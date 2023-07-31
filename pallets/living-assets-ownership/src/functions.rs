//! Contains helper and utility functions of the pallet
use super::*;
use frame_support::{
	dispatch::DispatchResult,
	sp_runtime::traits::{CheckedAdd, One},
};

impl<T: Config> Pallet<T> {
	/// Create a new collection
	///
	/// If this operation would result in an overflow, the function returns early
	/// with an [Error::CollectionIdOverflow].
	///
	/// Finally, if it is successful, the function emits a
	/// [Event::CollectionCreated] event and returns `Ok(())`.
	///
	/// # Arguments
	///
	/// * `who` - The account ID of the new collection's owner.
	///
	/// # Return
	///
	/// Returns a [DispatchResult] indicating the outcome of the operation. If the
	/// operation was successful, the function returns `Ok(())`. If the operation
	/// was not successful, the function returns `Err(e)`, where `e` is the error
	/// that occurred.
	pub fn do_create_collection(who: T::AccountId) -> DispatchResult {
		// Retrieve the current collection count to use as the new collection's ID
		let collection_id = Self::collection_counter();

		// Insert a new entry into the OwnerOfCollection map, mapping the new
		// collection's ID to the owner's account ID
		OwnerOfCollection::<T>::insert(collection_id, &who);

		// Attempt to increment the collection counter by 1. If this operation
		// would result in an overflow, return early with an error
		let counter =
			collection_id.checked_add(&One::one()).ok_or(Error::<T>::CollectionIdOverflow)?;
		CollectionCounter::<T>::put(counter);

		// Emit an event indicating that a new collection was created
		Self::deposit_event(Event::CollectionCreated { collection_id, who });

		// Return Ok to indicate that the operation was successful
		Ok(())
	}
}
