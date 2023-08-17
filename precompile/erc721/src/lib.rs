//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput};
use frame_support::pallet_prelude::*;
use pallet_living_assets_ownership::{address_to_collection_id, traits::Erc721};
use parity_scale_codec::Encode;
use precompile_utils::{
	revert, succeed, Address, EvmDataWriter, EvmResult, FunctionModifier, PrecompileHandleExt,
};

use sp_core::{H160, U256};
use sp_std::{fmt::Debug, marker::PhantomData};

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
pub struct Erc721Precompile<AddressMapping, AccountId, AssetManager>(
	PhantomData<(AddressMapping, AccountId, AssetManager)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	AssetManager: Erc721<AccountId>;

impl<AddressMapping, AccountId, AssetManager> Precompile
	for Erc721Precompile<AddressMapping, AccountId, AssetManager>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	AssetManager: Erc721<AccountId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::TokenURI => FunctionModifier::View,
			Action::OwnerOf => FunctionModifier::View,
			Action::TransferFrom => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::TokenURI => Err(revert("not implemented")),
			Action::OwnerOf => {
				// get input data
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;
				let asset_id: U256 = input.read()?;

				let owner = Self::owner_of(asset_id, handle.code_address())?;

				Ok(succeed(EvmDataWriter::new().write(Address(owner)).build()))
			},
			Action::TransferFrom => {
				// get input data
				let mut input = handle.read_input()?;
				input.expect_arguments(3)?;
				let from: H160 = input.read::<Address>()?.into();
				let to: H160 = input.read::<Address>()?.into();
				let asset_id: U256 = input.read()?;

				// checks
				let owner = Self::owner_of(asset_id, handle.code_address())?;
				ensure!(owner == from, revert("sender must be the current owner"));
				ensure!(from != to, revert("sender and receiver cannot be the same"));
				ensure!(to != H160::zero(), revert("receiver cannot be zero address"));

				Ok((succeed(EvmDataWriter::new().write(asset_id).build())).into())
			},
		}
	}
}

impl<AddressMapping, AccountId, AssetManager>
	Erc721Precompile<AddressMapping, AccountId, AssetManager>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	AssetManager: Erc721<AccountId>,
{
	fn owner_of(asset_id: U256, code_address: H160) -> Result<H160, PrecompileFailure> {
		// collection id is encoded into the contract address
		let collection_id = match address_to_collection_id(code_address) {
			Ok(collection_id) => collection_id,
			Err(_) => return Err(revert("invalid collection address")),
		};
		match AssetManager::owner_of(collection_id, asset_id) {
			Ok(owner) => Ok(owner),
			Err(err) => Err(revert(err)),
		}
	}
}

#[cfg(test)]
mod tests;
