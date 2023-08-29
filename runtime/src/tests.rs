use core::str::FromStr;

use super::*;
use pallet_living_assets_ownership::traits::{AccountMapping, AssetIdToAddress};
use sp_core::U256;
use sp_runtime::AccountId32;

#[test]
fn account_mappping_type_zero_values() {
	type TestAccountMapping = <Runtime as pallet_living_assets_ownership::Config>::AccountMapping;

	assert_eq!(TestAccountMapping::into_h160(AccountId32::from([0u8; 32])), H160::zero());
	assert_eq!(TestAccountMapping::into_account_id(H160::zero()), AccountId32::from([0u8; 32]));
}

#[test]
fn account_mappping_type_max_values() {
	type TestAccountMapping = <Runtime as pallet_living_assets_ownership::Config>::AccountMapping;
	assert_eq!(
		TestAccountMapping::into_h160(AccountId32::from([0xFFu8; 32])),
		H160::from([0xFFu8; 20])
	);
	assert_eq!(
		TestAccountMapping::into_account_id(H160::from([0xFFu8; 20])),
		AccountId32::from_str("000000000000000000000000ffffffffffffffffffffffffffffffffffffffff")
			.unwrap()
	);
}
#[test]
fn asset_id_to_address_type_zero_values() {
	type TestAssetIdToAddress =
		<Runtime as pallet_living_assets_ownership::Config>::AssetIdToAddress;

	assert_eq!(TestAssetIdToAddress::initial_owner(U256::from(0)), AccountId32::from([0u8; 32]));
}

#[test]
fn asset_id_to_address_type_max_values() {
	type TestAssetIdToAddress =
		<Runtime as pallet_living_assets_ownership::Config>::AssetIdToAddress;
	assert_eq!(
		TestAssetIdToAddress::initial_owner(U256::max_value()),
		AccountId32::from_str("000000000000000000000000ffffffffffffffffffffffffffffffffffffffff")
			.unwrap()
	);
}
#[test]
fn asset_id_to_address_two_assets_same_owner() {
	type TestAssetIdToAddress =
		<Runtime as pallet_living_assets_ownership::Config>::AssetIdToAddress;
	assert_eq!(
		TestAssetIdToAddress::initial_owner(U256::max_value()),
		AccountId32::from_str("000000000000000000000000ffffffffffffffffffffffffffffffffffffffff")
			.unwrap()
	);

	// create two different assets
	let asset1 =
		U256::from(hex::decode("01C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice());
	let asset2 =
		U256::from(hex::decode("03C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice());
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

	assert_eq!(
		TestAssetIdToAddress::initial_owner(asset1),
		AccountId32::from_str("000000000000000000000000ffffffffffffffffffffffffffffffffffffffff")
			.unwrap()
	);
	assert_eq!(
		TestAssetIdToAddress::initial_owner(asset2),
		AccountId32::from_str("000000000000000000000000ffffffffffffffffffffffffffffffffffffffff")
			.unwrap()
	);
}
