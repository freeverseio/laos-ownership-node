use core::str::FromStr;

use crate::{
	address_to_collection_id, collection_id_to_address, is_collection_address, mock::*, BaseURI,
	CollectionError, Event,
};
use frame_support::assert_ok;
use sp_core::H160;

type AccountId = <Test as frame_system::Config>::AccountId;

const ALICE: AccountId = 0x1234;

#[test]
fn base_uri_unexistent_collection_is_none() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssetsModule::collection_base_uri(0), None);
		assert_eq!(LivingAssetsModule::collection_base_uri(1), None);
	});
}

#[test]
fn create_new_collection_should_create_sequential_collections() {
	new_test_ext().execute_with(|| {
		// Check initial condition
		assert_eq!(LivingAssetsModule::collection_base_uri(0), None);

		let base_uri = base_uri_from_string("https://example.com/");

		// Iterate through the collections to be created
		for i in 0..3 {
			// Create the collection
			assert_ok!(LivingAssetsModule::create_collection(
				RuntimeOrigin::signed(ALICE),
				base_uri.clone()
			));

			// Assert that the collection was created with the expected URI
			assert_eq!(LivingAssetsModule::collection_base_uri(i).unwrap(), base_uri);
		}
	});
}

#[test]
fn should_set_base_uri_when_creating_new_collection() {
	let base_uri = base_uri_from_string("https://example.com/");
	new_test_ext().execute_with(|| {
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			base_uri.clone()
		));
		assert_eq!(LivingAssetsModule::collection_base_uri(0).unwrap(), base_uri);
	});
}

#[test]
fn create_new_collections_should_emit_events_with_collection_id_consecutive() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			base_uri_from_string("ciao")
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			base_uri_from_string("ciao")
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 1, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			base_uri_from_string("ciao")
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 2, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			base_uri_from_string("ciao")
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 3, who: ALICE }.into());
	});
}

#[test]
fn test_collection_id_to_address() {
	let collection_id = 5;
	let expected_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	assert_eq!(collection_id_to_address(collection_id), expected_address);
}

#[test]
fn invalid_collection_address_should_error() {
	let address = H160::from_str("8000000000000000000000000000000000000005").unwrap();
	let error = address_to_collection_id(address).unwrap_err();
	assert_eq!(error, CollectionError::InvalidPrefix);
}

#[test]
fn valid_collection_address_should_return_collection_id() {
	let address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let collection_id = address_to_collection_id(address).unwrap();
	assert_eq!(collection_id, 5);
}

#[test]
fn test_is_collection_address_valid() {
	let collection_id = 1234567890;
	let address = collection_id_to_address(collection_id);

	assert!(is_collection_address(address));
}

#[test]
fn test_is_collection_address_invalid() {
	let invalid_address = H160([0u8; 20]);

	assert!(!is_collection_address(invalid_address));
}

fn base_uri_from_string(url: &str) -> BaseURI {
	let mut base_uri = BaseURI::new();
	base_uri.try_append(&mut url.to_string().into_bytes()).unwrap();
	base_uri
}
