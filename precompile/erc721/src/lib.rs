#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use frame_support::pallet_prelude::*;
use pallet_living_assets_ownership::{address_to_collection_id, CollectionId};
use precompile_utils::{
	revert, succeed, Address, Bytes, EvmDataWriter, EvmResult, FunctionModifier,
	PrecompileHandleExt,
};

use sp_core::{H160, U256};
use sp_std::{fmt::Debug, marker::PhantomData, vec, vec::Vec};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Get token URI
	TokenURI = "tokenURI(uint256)",
	/// Owner of
	OwnerOf = "ownerOf(uint256)",
	/// Transfer from
	TransferFrom = "transferFrom(address,address,uint256)",
}

/// Wrapper for the precompile function.
pub struct Erc721Precompile<AssetManager, AddressMapping>(
	PhantomData<(AssetManager, AddressMapping)>,
);

impl<AssetManager, AddressMapping> Precompile for Erc721Precompile<AssetManager, AddressMapping>
where
	AssetManager: pallet_living_assets_ownership::traits::Erc721,
	AddressMapping: pallet_evm::AddressMapping<AssetManager::AccountId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(handle.code_address())
			.map_err(|_| revert("invalid collection address"))?;

		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::TokenURI => FunctionModifier::View,
			Action::OwnerOf => FunctionModifier::View,
			Action::TransferFrom => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::TokenURI => Self::token_uri(collection_id, handle),
			Action::OwnerOf => Self::owner_of(collection_id, handle),
			Action::TransferFrom => Self::transfer_from(handle),
		}
	}
}

impl<AssetManager, AddressMapping> Erc721Precompile<AssetManager, AddressMapping>
where
	AssetManager: pallet_living_assets_ownership::traits::Erc721,
	AddressMapping: pallet_evm::AddressMapping<AssetManager::AccountId>,
{
	fn owner_of(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let asset_id: U256 = input.read()?;

		let owner = AssetManager::owner_of(collection_id, asset_id).map_err(|err| revert(err))?;
		Ok(succeed(EvmDataWriter::new().write(Address(owner)).build()))
	}

	fn token_uri(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let asset_id: U256 = input.read()?;

		let uri = AssetManager::token_uri(collection_id, asset_id).map_err(|err| revert(err))?;
		Ok(succeed(EvmDataWriter::new().write(Bytes(uri)).build()))
	}

	fn transfer_from(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// get input data
		let mut input = handle.read_input()?;
		input.expect_arguments(3)?;
		let from: H160 = input.read::<Address>()?.into();
		let to: H160 = input.read::<Address>()?.into();
		let asset_id: U256 = input.read()?;

		// collection id is encoded into the contract address
		let collection_id = match address_to_collection_id(handle.code_address()) {
			Ok(collection_id) => collection_id,
			Err(_) => return Err(revert("invalid collection address")),
		};

		// get current owner
		let result = Self::owner_of(collection_id, handle)?;
		let owner: H160 = match TryInto::<Vec<u8>>::try_into(result.output) {
			Ok(value) => H160::from_slice(&value.as_slice()[12..32]),
			Err(_) => return Err(revert("error getting owner")),
		};
		let caller = handle.context().caller;

		// checks
		ensure!(owner == caller, revert("caller must be the current owner"));
		ensure!(owner == from, revert("sender must be the current owner"));
		ensure!(from != to, revert("sender and receiver cannot be the same"));
		ensure!(to != H160::zero(), revert("receiver cannot be zero address"));

		match AssetManager::transfer_from(collection_id, from, to, asset_id) {
			Ok(_) => Ok(succeed(vec![])),
			Err(err) => Err(revert(err)),
		}
	}
}

#[cfg(test)]
mod tests;
