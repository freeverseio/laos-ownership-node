//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]
use fp_evm::{
	Precompile, PrecompileHandle, PrecompileOutput,
};
use parity_scale_codec::Encode;
use precompile_utils::{EvmResult, FunctionModifier, PrecompileHandleExt};

use sp_std::{fmt::Debug, marker::PhantomData};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Get tocken URI
	TockenURI = "tokenURI(uint256)",
}

/// Wrapper for the precompile function.
pub struct CollectionManagerPrecompile<AddressMapping, AccountId>(
	PhantomData<(AddressMapping, AccountId)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug;

impl<AddressMapping, AccountId> Precompile
	for CollectionManagerPrecompile<AddressMapping, AccountId>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::TockenURI => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::TockenURI => {
				todo!()
			},
		}
	}
}


