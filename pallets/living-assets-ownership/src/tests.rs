use core::str::FromStr;

use crate::{
	address_to_collection_id, collection_id_to_address, is_collection_address,
	mock::*,
	traits::{CollectionManager, Erc721},
	Event,
};
use frame_support::assert_ok;
use sp_core::{H160, U256};

type AccountId = <Test as frame_system::Config>::AccountId;

const ALICE: AccountId = 0x1234;

#[test]
fn owner_of_unexistent_collection_is_none() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssetsModule::owner_of_collection(0), None);
		assert_eq!(LivingAssetsModule::owner_of_collection(1), None);
	});
}

#[test]
fn create_new_collection() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssetsModule::owner_of_collection(0), None);

		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		assert_eq!(LivingAssetsModule::owner_of_collection(0).unwrap(), ALICE);
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		assert_eq!(LivingAssetsModule::owner_of_collection(1).unwrap(), ALICE);
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		assert_eq!(LivingAssetsModule::owner_of_collection(2).unwrap(), ALICE);
	});
}

#[test]
fn create_new_collections_should_emit_events_with_collection_id_consecutive() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 1, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 2, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 3, who: ALICE }.into());
	});
}

#[test]
fn living_assets_ownership_trait_create_new_collection() {
	new_test_ext().execute_with(|| {
		let result = <LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE);
		assert_ok!(result);
		assert_eq!(LivingAssetsModule::owner_of_collection(0).unwrap(), ALICE);
	});
}

#[test]
fn living_assets_ownership_trait_owner_of_unexistent_collection_is_none() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::owner_of_collection(0),
			None
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::owner_of_collection(1),
			None
		);
	});
}

#[test]
fn living_assets_ownership_trait_create_new_collection_should_emit_an_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE));
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: ALICE }.into());
	});
}

#[test]
fn living_assets_ownership_trait_id_of_new_collection_should_be_consecutive() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap(),
			0
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap(),
			1
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap(),
			2
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap(),
			3
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap(),
			4
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap(),
			5
		);
	});
}

#[test]
fn erc721_owner_of_asset_of_unexistent_collection() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			<LivingAssetsModule as Erc721>::owner_of(0, 2.into()),
			Err("Collection does not exist")
		);
	});
}

#[test]
fn erc721_owner_of_asset_of_collection() {
	new_test_ext().execute_with(|| {
		let collection_id =
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap();
		assert_eq!(
			<LivingAssetsModule as Erc721>::owner_of(collection_id, 2.into()).unwrap(),
			H160::from_low_u64_be(0x0000000000000002)
		);
	});
}

#[test]
fn erc721_owner_of_coincides_for_asset_id_larger_than_160b() {
	new_test_ext().execute_with(|| {
		let collection_id =
			<LivingAssetsModule as CollectionManager<AccountId>>::create_collection(ALICE).unwrap();
		let expected_owner = H160::from_str("931D387731BBBC988B312206C74F77D004D6B84B");
		// build two asset ids by prepending "0x1" and "0x2" to the same owner:
		let asset_id_1 = U256::from("0x1931D387731BBBC988B312206C74F77D004D6B84B");
		let asset_id_2 = U256::from("0x2931D387731BBBC988B312206C74F77D004D6B84B");

		assert_eq!(
			<LivingAssetsModule as Erc721>::owner_of(collection_id, asset_id_1).unwrap(),
			expected_owner.unwrap()
		);
		assert_eq!(
			<LivingAssetsModule as Erc721>::owner_of(collection_id, asset_id_2).unwrap(),
			expected_owner.unwrap()
		);
	});
}

#[test]
fn test_collection_id_to_address() {
	let collection_id: u64 = 5;
	let expected_address = H160::from_str("8000000000000000000000000000000000000005").unwrap();
	assert_eq!(collection_id_to_address(collection_id), expected_address);
}

#[test]
fn test_address_to_collection_id() {
	let address = H160::from_str("8000000000000000000000000000000000000005").unwrap();
	let collection_it = address_to_collection_id(address);
	assert_eq!(collection_it, 5);
}

#[test]
fn check_for_erc721_addresses() {
	assert!(!is_collection_address(
		H160::from_str("0x1000000000000000000000000000000000000001").unwrap()
	));
	assert!(is_collection_address(
		H160::from_str("0x8000000000000000000000000000000000000000").unwrap()
	));
	assert!(is_collection_address(
		H160::from_str("0x8000000000000000000000000000000000000001").unwrap()
	));
	assert!(is_collection_address(
		H160::from_str("0x8000000000000000000000000000000000000002").unwrap()
	));
	assert!(is_collection_address(
		H160::from_str("0x8000000000000000000000000000000000000003").unwrap()
	));
	assert!(is_collection_address(
		H160::from_str("0x80000000000000000000000000000000ffffffff").unwrap()
	));
	assert!(!is_collection_address(
		H160::from_str("0x7fffffffffffffffffffffffffffffffffffffff").unwrap()
	));
	assert!(is_collection_address(
		H160::from_str("0xffffffffffffffffffffffffffffffffffffffff").unwrap()
	));
}
