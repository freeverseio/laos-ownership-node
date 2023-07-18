//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{Precompile, PrecompileHandle};
use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
use pallet_living_assets_ownership::{
	Call as LivingAssetsOwnershipCall, Pallet as LivingAssetsOwnership,
};
use precompile_utils::EvmResult;
use precompile_utils_macro::precompile;
use sp_core::{H160, H256};
use sp_std::marker::PhantomData;

// type CollectionIdOf<T> = <T as pallet_living_assets_ownership::Config>::CollectionId;

/// A precompile to wrap the functionality from pallet-living-assets-ownership.
pub struct LivingAssetsOwnershipPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
#[precompile_utils::test_concrete_types(mock::Runtime)]
impl<Runtime> LivingAssetsOwnershipPrecompile<Runtime>
where
	Runtime: pallet_living_assets_ownership::Config + frame_system::Config + pallet_evm::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<LivingAssetsOwnershipCall<Runtime>>,
	Runtime::Hash: From<H256>,
	Runtime::AccountId: Into<H160>,
{
	/// Dispatches `create_collection` call.
	#[precompile::public("createCollection(uint64,address)")]
	#[precompile::public("create_collection(uint64,address)")]
	fn create_collection(
		handle: &mut impl PrecompileHandle,
		collection_id: CollectionIdOf<T>,
	) -> EvmResult {
	}

	/// Returns owner of the collection, if it exists.
	#[precompile::public("ownerOfCollection(uint64)")]
	#[precompile::public("owner_of_collection(uint64)")]
	fn owner_of_collection(handle: &mut impl PrecompileHandle) -> EvmResult {}
}
