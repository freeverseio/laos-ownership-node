use super::*;
use frame_support::{
	pallet_prelude::OptionQuery, storage_alias, traits::GetStorageVersion, weights::Weight,
	Blake2_128Concat,
};
use log::info;
use sp_core::U256;

// only contains V1 storage format
pub mod v1 {
	use super::*;

	#[storage_alias]
	pub(super) type AssetOwner<T: Config> = StorageMap<
		Pallet<T>,
		Blake2_128Concat,
		U256,
		<T as frame_system::Config>::AccountId,
		OptionQuery,
	>;
}

// contains checks and transforms storage to V2 format
pub fn migrate_to_v2<T: Config>() -> Weight {
	let onchain_version = Pallet::<T>::on_chain_storage_version();
	if onchain_version < 2 {
		Pallet::<T>::deposit_event().iter().for_each(|event| {
			info!(" >>> Event: {:?}", event);
		});
		Weight::zero()
	} else {
		info!(" >>> Unused migration!");
		// We don't do anything here.
		Weight::zero()
	}
}
