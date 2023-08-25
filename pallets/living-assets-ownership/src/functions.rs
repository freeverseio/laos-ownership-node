//! Contains helper and utility functions of the pallet
use super::*;
use frame_support::sp_runtime::traits::One;
use sp_core::{H160, U256};

impl<T: Config> Pallet<T> {
	/// See [Self::create_collection]
	pub fn do_create_collection(
		who: T::AccountId,
		base_uri: BaseURI<T>,
	) -> Result<CollectionId, Error<T>> {
		// Retrieve the current collection count to use as the new collection's ID
		let collection_id = Self::collection_counter();

		CollectionBaseURI::<T>::insert(collection_id, base_uri);

		// Attempt to increment the collection counter by 1. If this operation
		// would result in an overflow, return early with an error
		let counter =
			collection_id.checked_add(One::one()).ok_or(Error::<T>::CollectionIdOverflow)?;
		CollectionCounter::<T>::put(counter);

		Self::deposit_event(Event::CollectionCreated { collection_id, who });

		Ok(collection_id)
	}
}

pub struct AssetId(pub U256);
impl From<AssetId> for H160 {
	fn from(asset_id: AssetId) -> H160 {
		let mut bytes = [0u8; 20];
		let asset_id_bytes: [u8; 32] = asset_id.0.into();
		bytes.copy_from_slice(&asset_id_bytes[asset_id_bytes.len() - 20..]);
		H160::from(bytes)
	}
}

// TODO maybe collection also can be refactored

pub fn convert_asset_id_to_owner(value: U256) -> H160 {
	// TODO add tests so I use the next one instead of this
	let mut bytes = [0u8; 20];
	let value_bytes: [u8; 32] = value.into();
	bytes.copy_from_slice(&value_bytes[value_bytes.len() - 20..]);
	H160::from(bytes)
}

// TODO add docs for that, the objective of the PR, talk about the signing part and origin, todo moonbeam unified account, talk about this is not reacheable from polkadot js
// TODO another refactor: refactor/add_config_to_erc721_trait
// TODO remove branch change_requests, stash, feature/transfer_from_in_living_asset_pallet
pub trait AccountMapping<T: Config> {
	fn initial_owner(asset_id: U256) -> T::AccountId; // TODO if next one is fulfill maybe I can use impl From<AssetId> for H160 {
	fn into_h160(account_id: T::AccountId) -> H160; // TODO this and next one if in exchange I get a response maybe I can use
	fn from_h160(account_id: H160) -> T::AccountId;
}

#[cfg(test)]
mod tests {
	use crate::functions::{convert_asset_id_to_owner, AssetId}; // TODO unify this use or not AssetId
	use sp_core::{H160, U256};

	#[test]
	fn check_convert_asset_id_to_owner() {
		let value: H160 = AssetId(U256::from(5)).into();
		let expected_address = H160::from_low_u64_be(5);
		assert_eq!(value, expected_address);
	}

	#[test]
	fn check_two_assets_same_owner() {
		// create two different assets
		let asset1 = U256::from(
			hex::decode("01C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice(),
		);
		let asset2 = U256::from(
			hex::decode("03C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice(),
		);
		assert_ne!(asset1, asset2);

		// check asset in decimal format
		assert_eq!(
			U256::from_str_radix("01C0F0f4ab324C46e55D02D0033343B4Be8A55532d", 16).unwrap(),
			U256::from_dec_str("2563001357829637001682277476112176020532353127213").unwrap()
		);
		assert_eq!(
			U256::from_str_radix("03C0F0f4ab324C46e55D02D0033343B4Be8A55532d", 16).unwrap(),
			U256::from_dec_str("5486004632491442838089647141544742059844218213165").unwrap()
		);

		let mut owner = [0u8; 20];
		owner.copy_from_slice(
			hex::decode("C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice(),
		);
		let expected_address = H160::from(owner);
		assert_eq!(convert_asset_id_to_owner(asset1), expected_address);
		assert_eq!(convert_asset_id_to_owner(asset2), expected_address);
	}
}
