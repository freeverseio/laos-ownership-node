//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{ExitError, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput};
use pallet_living_assets_ownership::{address_to_collection_id, traits::Erc721};
use parity_scale_codec::Encode;
use precompile_utils::{
	succeed, Address, EvmDataWriter, EvmResult, FunctionModifier, PrecompileHandleExt,
};

use sp_core::U256;
use sp_std::{fmt::Debug, marker::PhantomData};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Get tocken URI
	TockenURI = "tokenURI(uint256)",
	/// Owner of
	OwnerOf = "ownerOf(uint256)",
}

/// Wrapper for the precompile function.
pub struct Erc721Precompile<AddressMapping, AccountId, AssetManager>(
	PhantomData<(AddressMapping, AccountId, AssetManager)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	AssetManager: Erc721;

impl<AddressMapping, AccountId, AssetManager> Precompile
	for Erc721Precompile<AddressMapping, AccountId, AssetManager>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	AssetManager: Erc721,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::TockenURI => FunctionModifier::NonPayable,
			Action::OwnerOf => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::TockenURI => {
				todo!()
			},
			Action::OwnerOf => {
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;

				let asset_id: U256 = input.read()?;

				// collection id is encoded into the contract address
				let collection_id = match address_to_collection_id(handle.code_address()) {
					Ok(collection_id) => collection_id,
					Err(_) => {
						return Err(PrecompileFailure::Error {
							exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(
								"invalid collection address",
							)),
						})
					},
				};
				match AssetManager::owner_of(collection_id, asset_id) {
					Ok(owner) => Ok(succeed(EvmDataWriter::new().write(Address(owner)).build())),
					Err(err) => Err(PrecompileFailure::Error {
						exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(err)),
					}),
				}
			},
		}
	}
}

#[cfg(test)]
mod tests;
