// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]

use fp_evm::{
	ExitError, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult,
};
use frame_support::log;
use pallet_living_assets_ownership::LivingAssetsOwnership;
use sp_arithmetic::traits::BaseArithmetic;
// use precompile_utils::{Address, EvmResult, FunctionModifier, PrecompileHandleExt};
use sp_runtime::SaturatedConversion;

use sp_core::H160;
use sp_std::marker::PhantomData;

/// Wrapper for the precompile function.
pub struct LivingAssetsOwnershipPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>(
	PhantomData<(AddressMapping, AccountId, CollectionId, LivingAssets)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	CollectionId: BaseArithmetic,
	LivingAssets: LivingAssetsOwnership<AccountId, CollectionId>;

impl<AddressMapping, AccountId, CollectionId, LivingAssets>
	LivingAssetsOwnershipPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	CollectionId: BaseArithmetic,
	LivingAssets: LivingAssetsOwnership<AccountId, CollectionId>,
{
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<AddressMapping, AccountId, CollectionId, LivingAssets> fp_evm::Precompile
	for LivingAssetsOwnershipPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	CollectionId: BaseArithmetic,
	LivingAssets: LivingAssetsOwnership<AccountId, CollectionId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		log::trace!(target: "ds-precompile", "Execute input = {:?}", handle.input());

		let context = handle.context();
		let input = handle.input();

		log::info!("Context: {:?}", context);

		// first read u64 from input
		let mut buffer = [0u8; 8];
		buffer.copy_from_slice(&input[..8]);
		let collection_id = u64::from_be_bytes(buffer).saturated_into();

		// then read address from input
		let mut buffer = [0u8; 20];
		buffer.copy_from_slice(&input[8..28]);
		let owner = AddressMapping::into_account_id(H160::from_slice(&buffer));

		if let Err(_) = LivingAssets::create_collection(collection_id, owner) {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(
					"Could net create collection",
				)),
			})
		}

		Ok(PrecompileOutput { exit_status: ExitSucceed::Returned, output: sp_std::vec::Vec::new() })
	}
}
