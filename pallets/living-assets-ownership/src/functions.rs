//! Contains helper and utility functions of the pallet
use super::*;
use frame_support::{
	ensure,
	sp_runtime::{
		traits::{CheckedAdd, One},
		DispatchResult,
	},
};

impl<T: Config> Pallet<T> {
	/// See [Self::create_collection]
	pub fn do_create_collection(
		collection_id: T::CollectionId,
		who: T::AccountId,
	) -> DispatchResult {
		ensure!(
			!OwnerOfCollection::<T>::contains_key(collection_id),
			Error::<T>::CollectionAlreadyExists
		);

		// Retrieve the current collection count
		let collection_id = Self::collection_counter();

		OwnerOfCollection::<T>::insert(collection_id, &who);

		// Increment collection counter by 1
		let counter =
			collection_id.checked_add(&One::one()).ok_or(Error::<T>::CollectionIdOverflow)?;
		CollectionCounter::<T>::put(counter);

		Self::deposit_event(Event::CollectionCreated { collection_id, who });

		Ok(())
	}
}
