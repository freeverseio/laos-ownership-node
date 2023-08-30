use crate::{self as pallet_livingassets_ownership, traits};
use core::fmt::Debug;
use frame_support::traits::{ConstU16, ConstU64};
use frame_support::{
	dispatch::Vec,
	pallet_prelude::*,
	sp_runtime::traits::Hash,
	traits::{fungible, OriginTrait},
};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::{ConstU32, H160, H256, U256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use std::fmt;

use sp_std::{boxed::Box, prelude::*};
// Substrate
use sp_core::{ecdsa, RuntimeDebug};
type Block = frame_system::mocking::MockBlock<Test>;
type Nonce = u32;
#[derive(
	Copy,
	Clone,
	// Eq,
	// PartialEq,
	Ord,
	PartialOrd,
	Default,
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
	// Debug,
	DebugNoBound,
	PartialEqNoBound,
	EqNoBound,
	derive_more::Display,
	Serialize,
	Deserialize,
)]
pub struct CustomAccountId(u64);
impl From<H160> for CustomAccountId {
	fn from(nonce: H160) -> Self {
		1 // TODO dummy value
	}
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		LivingAssetsModule: pallet_livingassets_ownership,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Block = Block;
	type Hash = H256;
	type Nonce = Nonce;
	type Hashing = BlakeTwo256;
	type AccountId = CustomAccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_livingassets_ownership::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AccountId = CustomAccountId;
	type BaseURILimit = ConstU32<256>;
	type AccountMapping = MockAccountMapping;
	type AssetIdToAddress = MockAssetIdToAddress;
}

pub struct MockAccountMapping;
impl traits::AccountMapping<CustomAccountId> for MockAccountMapping {
	fn into_h160(account_id: CustomAccountId) -> H160 {
		H160::from_low_u64_be(account_id)
	}
	fn into_account_id(account_id: H160) -> CustomAccountId {
		H160::to_low_u64_be(&account_id)
	}
}

pub struct MockAssetIdToAddress;
impl traits::AssetIdToAddress<CustomAccountId> for MockAssetIdToAddress {
	fn initial_owner(asset_id: U256) -> CustomAccountId {
		let mut first_eight_bytes = [0u8; 8];
		let asset_id_bytes: [u8; 32] = asset_id.into();
		first_eight_bytes.copy_from_slice(&asset_id_bytes[asset_id_bytes.len() - 8..]);
		u64::from_be_bytes(first_eight_bytes).into()
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	RuntimeGenesisConfig::default().build_storage().unwrap().into()
}
