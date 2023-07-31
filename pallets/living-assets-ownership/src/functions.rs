//! Contains helper and utility functions of the pallet
use super::*;
use frame_support::{
	dispatch::DispatchResult,
	sp_runtime::traits::{CheckedAdd, One},
};

impl<T: Config> Pallet<T> {
	/// Creates a new collection.
	///
	/// The function first retrieves the current collection count, which will be used as the new collection's ID.
	/// It then inserts a new entry into the `OwnerOfCollection` map, associating the new collection's ID with the specified owner's account ID.
	///
	/// The collection counter is then incremented by one, using checked addition to prevent overflow. If an overflow would occur, the function returns early with an [Error::CollectionIdOverflow].
	///
	/// If successful, the function emits a [Event::CollectionCreated] event, indicating that the new collection was created, and returns `Ok(())`.
	///
	/// # Arguments
	///
	/// * `who` - The account ID of the new collection's owner. This account will be associated with the newly created collection.
	///
	/// # Return
	///
	/// Returns a [DispatchResult] that signifies the outcome of the operation:
	/// - `Ok(())` if the operation was successful, indicating that the new collection was created without issue.
	/// - `Err(e)` if the operation failed, where `e` is the error that occurred, such as an overflow error.
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
