//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]
use fp_evm::{
	ExitError, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput,
	PrecompileResult, PrecompileSet,
};
use frame_support::log;
use pallet_living_assets_ownership::LivingAssetsOwnership;
use parity_scale_codec::Codec;
// use precompile_utils::{Address, EvmResult, FunctionModifier, PrecompileHandleExt};
use sp_runtime::SaturatedConversion;

use sp_core::H160;
use sp_std::marker::PhantomData;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// #[precompile_utils::generate_function_selector]
// #[derive(Debug, PartialEq)]
// pub enum Action {
// 	/// Create a new collection
// 	CreateCollection = "createCollection(uint64,address)",
// 	/// Get owner of the collection
// 	OwnerOfCollection = "owner_of_collection(uint64)",
// }

/// Wrapper for the precompile function.
pub struct LivingAssetsOwnershipPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> LivingAssetsOwnershipPrecompile<Runtime> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<Runtime> fp_evm::Precompile for LivingAssetsOwnershipPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_living_assets_ownership::Config,
	Runtime::AccountId: From<H160> + Codec,
	<Runtime as pallet_living_assets_ownership::Config>::CollectionId: SaturatedConversion,
{
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		log::trace!(target: "ds-precompile", "Execute input = {:?}", handle.input());

		let context = handle.context();
		let input = handle.input();

		log::info!("Context: {:?}", context);

		// first read u64 from input
		let mut buffer = [0u8; 8];
		buffer.copy_from_slice(&input[..8]);
		let collection_id = u64::from_be_bytes(buffer).saturated_into();

		// then read address from input
		let mut buffer = [0u8; 20];
		buffer.copy_from_slice(&input[8..28]);
		let owner = H160::from_slice(&buffer).into();

		if let Err(_) = <pallet_living_assets_ownership::Pallet<Runtime> as LivingAssetsOwnership<
			Runtime,
		>>::create_collection(collection_id, owner)
		{
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(
					"Could net create collection",
				)),
			})
		}

		Ok(PrecompileOutput { exit_status: ExitSucceed::Returned, output: sp_std::vec::Vec::new() })

		// let selector = handle.read_selector()?;

		// handle.check_function_modifier(match selector {
		// 	Action::OwnerOfCollection => FunctionModifier::View,
		// 	_ => FunctionModifier::NonPayable,
		// })?;

		// match selector {
		// 	// read storage
		// 	Action::OwnerOfCollection => {
		// 		let mut input = handle.read_input()?;
		// 		input.expect_arguments(1)?;

		// 		if let Some(owner) =
		// 			<pallet_living_assets_ownership::Pallet<Runtime> as LivingAssetsOwnership<
		// 				Runtime,
		// 			>>::owner_of_collection(input.read::<u64>()?.saturated_into())
		// 		{
		// 			Ok(PrecompileOutput {
		// 				exit_status: ExitSucceed::Returned,
		// 				output: owner.encode(),
		// 			})
		// 		} else {
		// 			Ok(PrecompileOutput {
		// 				exit_status: ExitSucceed::Stopped,
		// 				output: sp_std::vec::Vec::new(),
		// 			})
		// 		}
		// 	},
		// 	// write storage
		// 	Action::CreateCollection => {
		// 		let mut input = handle.read_input()?;
		// 		input.expect_arguments(2)?;

		// 		let collection_id = input.read::<u64>()?.saturated_into();
		// 		let owner = input.read::<Address>()?.0.into();

		// 		if let Err(_) =
		// 			<pallet_living_assets_ownership::Pallet<Runtime> as LivingAssetsOwnership<
		// 				Runtime,
		// 			>>::create_collection(collection_id, owner)
		// 		{
		// 			return Err(PrecompileFailure::Error {
		// 				exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(
		// 					"Could net create collection",
		// 				)),
		// 			})
		// 		}

		// 		Ok(PrecompileOutput {
		// 			exit_status: ExitSucceed::Returned,
		// 			output: sp_std::vec::Vec::new(),
		// 		})
		// 	},
	}
}

impl<Runtime> PrecompileSet for LivingAssetsOwnershipPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_living_assets_ownership::Config,
	Runtime::AccountId: From<H160> + Codec,
	<Runtime as pallet_living_assets_ownership::Config>::CollectionId: SaturatedConversion,
	LivingAssetsOwnershipPrecompile<Runtime>: pallet_evm::Precompile,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match <Self as pallet_evm::Precompile>::execute(handle) {
			Ok(output) => Some(Ok(output)),
			Err(err) => {
				log::error!("Precompile error: {:?}", err);
				None
			},
		}
	}

	fn is_precompile(&self, _address: H160, _remaining_gas: u64) -> fp_evm::IsPrecompileResult {
		fp_evm::IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 }
	}
}
