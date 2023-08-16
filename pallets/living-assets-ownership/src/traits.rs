use crate::{BaseURI, CollectionId};
use sp_core::{H160, U256};

/// The `CollectionManager` trait provides an interface for managing collections in a decentralized system.
///
/// A collection is a container for non-fungible assets, uniquely identified by a `collection_id`. The system allows
/// for the creation and ownership management of these collections.
///
/// # Methods
///
/// - `owner_of_collection`: Retrieve the owner of a specified collection.
/// - `create_collection`: Create a new collection and assign it to an owner.
pub trait CollectionManager<AccountId> {
	type Error;

	/// Retrieves the base uri of the specified collection.
	///
	/// # Arguments
	///
	/// * `collection_id` - The unique identifier of the collection.
	///
	/// # Returns
	///
	/// The base URI associated with the specified collection or `None` if the collection doesn't exist.
	fn base_uri(collection_id: CollectionId) -> Option<BaseURI>;

	/// Creates a new collection and assigns it to the specified owner.
	///
	/// # Arguments
	///
	/// * `owner` - The account ID of the new collection's owner.
	///
	/// # Returns
	///
	/// A result containing the `collection_id` of the newly created collection or an error.
	fn create_collection(owner: AccountId, base_uri: BaseURI) -> Result<CollectionId, Self::Error>;
}

/// The `Erc721` trait provides an interface for handling ERC721 tokens in a blockchain environment.
///
/// ERC721 tokens are a standard for representing ownership of unique items on the Ethereum blockchain.
///
/// # Methods
///
/// - `owner_of`: Retrieve the owner of a specific asset within a collection.
pub trait Erc721 {
	type Error;

	/// Retrieves the owner of a specific asset within the specified collection.
	///
	/// # Arguments
	///
	/// * `collection_id` - The unique identifier for the collection.
	/// * `asset_id` - The unique identifier for the asset within the collection.
	///
	/// # Returns
	///
	/// The Ethereum address (`H160`) of the asset's owner or an error.
	fn owner_of(collection_id: CollectionId, asset_id: U256) -> Result<H160, Self::Error>;
}

#[cfg(test)]
mod tests {
	use frame_support::{assert_err, assert_ok};

	use super::*;
	use crate::{mock::*, Erc721Error, Event};

	type AccountId = <Test as frame_system::Config>::AccountId;
	const ALICE: AccountId = 0x1234;

	#[test]
	fn base_uri_of_unexistent_collection_is_none() {
		new_test_ext().execute_with(|| {
			assert_eq!(<LivingAssetsModule as CollectionManager<AccountId>>::base_uri(0), None);
			assert_eq!(<LivingAssetsModule as CollectionManager<AccountId>>::base_uri(1), None);
		});
	}

	#[test]
	fn create_new_collection_should_emit_an_event() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			assert_ok!(<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
				ALICE,
				BaseURI::default(),
			));
			System::assert_last_event(
				Event::CollectionCreated { collection_id: 0, who: ALICE }.into(),
			);
		});
	}

	#[test]
	fn living_assets_ownership_trait_id_of_new_collection_should_be_consecutive() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				0
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				1
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				2
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				3
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				4
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				5
			);
		});
	}

	#[test]
	fn living_assets_ownership_trait_should_set_base_uri_when_creating_new_collection() {
		let base_uri = BaseURI::try_from("https://example.com/".as_bytes().to_vec()).unwrap();

		new_test_ext().execute_with(|| {
			assert_ok!(<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
				ALICE,
				base_uri.clone()
			));
			assert_eq!(LivingAssetsModule::collection_base_uri(0).unwrap(), base_uri);
		});
	}

	#[test]
	fn owner_of_asset_of_unexistent_collection_should_error() {
		new_test_ext().execute_with(|| {
			let result = <LivingAssetsModule as Erc721>::owner_of(0, 2.into());
			assert_err!(result, Erc721Error::UnexistentCollection);
		});
	}

	#[test]
	fn erc721_owner_of_asset_of_collection() {
		new_test_ext().execute_with(|| {
			let collection_id =
				<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(
					ALICE,
					BaseURI::default(),
				)
				.unwrap();
			assert_eq!(
				<LivingAssetsModule as Erc721>::owner_of(collection_id, 2.into()).unwrap(),
				H160::from_low_u64_be(0x0000000000000002)
			);
		});
	}
}
