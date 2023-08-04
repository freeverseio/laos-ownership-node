//! Contains helper and utility functions of the pallet
use super::*;
use frame_support::sp_runtime::traits::One;
use sp_core::{Encode, H160, U256};

impl<T: Config> Pallet<T> {
	/// See [Self::create_collection]
	pub fn do_create_collection(who: T::AccountId) -> Result<CollectionId, &'static str> {
		// Retrieve the current collection count to use as the new collection's ID
		let collection_id = Self::collection_counter();

		// Insert a new entry into the OwnerOfCollection map, mapping the new
		// collection's ID to the owner's account ID
		OwnerOfCollection::<T>::insert(collection_id, &who);

		// Attempt to increment the collection counter by 1. If this operation
		// would result in an overflow, return early with an error
		let counter =
			collection_id.checked_add(One::one()).ok_or(Error::<T>::CollectionIdOverflow)?;
		CollectionCounter::<T>::put(counter);

		Self::deposit_event(Event::CollectionCreated { collection_id, who });

		Ok(collection_id)
	}
}

pub fn convert_u256_to_h160(value: U256) -> H160 {
	let bytes = value.encode();
	H160::from_slice(&bytes[12..32])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_convert_u256_to_h160() {
		let value = U256::from(5);
		let expected_address = H160::from_low_u64_be(5);
		assert_eq!(convert_u256_to_h160(value), expected_address);
	}
}
