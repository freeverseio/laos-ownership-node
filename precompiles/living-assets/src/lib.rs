//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]
use fp_evm::{ExitError, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput};
use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	log,
};
use pallet_evm::Precompile;
use pallet_living_assets_ownership::{Call as LivingAssetsOwnershipCall, LivingAssetsOwnership};
use parity_scale_codec::{Codec, Encode};
use precompile_utils::{Address, EvmResult, FunctionModifier, PrecompileHandleExt};
use sp_runtime::SaturatedConversion;

use sp_core::{H160, U256};
use sp_std::marker::PhantomData;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create a new collection
	CreateCollection = "createCollection(uint64,address)",
	/// Get owner of the collection
	OwnerOfCollection = "owner_of_collection(uint64)",
}

/// Wrapper for the precompile function.
pub struct LivingAssetsOwnershipPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> LivingAssetsOwnershipPrecompile<Runtime> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<Runtime> Precompile for LivingAssetsOwnershipPrecompile<Runtime>
where
	Runtime: pallet_living_assets_ownership::Config + frame_system::Config + pallet_evm::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<LivingAssetsOwnershipCall<Runtime>>,
	Runtime::Hash: From<U256>,
	Runtime::AccountId: From<H160> + Codec,
	<Runtime as pallet_living_assets_ownership::Config>::CollectionId: SaturatedConversion,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		log::trace!(target: "ds-precompile", "Execute input = {:?}", handle.input());
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::OwnerOfCollection => FunctionModifier::View,
			_ => FunctionModifier::NonPayable,
		})?;

		match selector {
			// read storage
			Action::OwnerOfCollection => {
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;

				if let Some(owner) =
					<pallet_living_assets_ownership::Pallet<Runtime> as LivingAssetsOwnership<
						Runtime,
					>>::owner_of_collection(input.read::<u64>()?.saturated_into())
				{
					Ok(PrecompileOutput {
						exit_status: ExitSucceed::Returned,
						output: owner.encode(),
					})
				} else {
					Ok(PrecompileOutput {
						exit_status: ExitSucceed::Stopped,
						output: sp_std::vec::Vec::new(),
					})
				}
			},
			// write storage
			Action::CreateCollection => {
				let mut input = handle.read_input()?;
				input.expect_arguments(2)?;

				let collection_id = input.read::<u64>()?.saturated_into();
				let owner = input.read::<Address>()?.0.into();

				if let Err(_) =
					<pallet_living_assets_ownership::Pallet<Runtime> as LivingAssetsOwnership<
						Runtime,
					>>::create_collection(collection_id, owner)
				{
					return Err(PrecompileFailure::Error {
						exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(
							"Could net create collection",
						)),
					})
				}

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: sp_std::vec::Vec::new(),
				})
			},
		}
	}
}
