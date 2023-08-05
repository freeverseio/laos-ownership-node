//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use pallet_living_assets_ownership::traits::Erc721;
use parity_scale_codec::Encode;
use precompile_utils::{succeed, EvmResult, FunctionModifier, PrecompileHandleExt};

use sp_core::H160;
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
			Action::OwnerOf => Ok(succeed(H160::zero().encode())),
		}
	}
}

#[cfg(test)]
mod tests;
