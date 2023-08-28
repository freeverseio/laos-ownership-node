use core::str::FromStr;

use super::*;
use pallet_living_assets_ownership::traits::AccountMapping;
use sp_core::U256;
use sp_runtime::AccountId32;

#[test]
fn account_mappping_type_zero_values() {
	type TestAccountMapping = <Runtime as pallet_living_assets_ownership::Config>::AccountMapping;

	assert_eq!(TestAccountMapping::initial_owner(U256::from(0)), AccountId32::from([0u8; 32]));
	assert_eq!(TestAccountMapping::into_h160(AccountId32::from([0u8; 32])), H160::zero());
	assert_eq!(TestAccountMapping::into_account_id(H160::zero()), AccountId32::from([0u8; 32]));
}

#[test]
fn account_mappping_type_max_values() {
	type TestAccountMapping = <Runtime as pallet_living_assets_ownership::Config>::AccountMapping;
	assert_eq!(
		TestAccountMapping::initial_owner(U256::max_value()),
		AccountId32::from_str("000000000000000000000000ffffffffffffffffffffffffffffffffffffffff")
			.unwrap()
	);
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
