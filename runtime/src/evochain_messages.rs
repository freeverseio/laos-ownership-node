// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Everything required to serve Evochain <-> OwnershipParachain messages.

// TODO: this is almost exact copy of `evochain_messages.rs` from Rococo runtime.
// Should be extracted to a separate crate and reused here.

use crate::{Runtime, WithEvochainMessagesInstance};

use bp_messages::LaneId;
use bridge_runtime_common::messages_xcm_extension::{
	LaneIdFromChainId, XcmBlobHauler, XcmBlobHaulerAdapter,
};
use frame_support::{parameter_types, weights::Weight};
use sp_core::Get;
use xcm_builder::HaulBlobExporter;

parameter_types! {
	/// Weight of 2 XCM instructions is for simple `Trap(42)` program, coming through bridge
	/// (it is prepended with `UniversalOrigin` instruction). It is used just for simplest manual
	/// tests, confirming that we don't break encoding somewhere between.
	///
	/// Weight credit for our test messages.
	///
	/// 2 XCM instructions is for simple `Trap(42)` program, coming through bridge
	/// (it is prepended with `UniversalOrigin` instruction).
	pub WeightCredit: Weight = crate::UnitWeightCost::get().saturating_mul(2);
}

/// Call-dispatch based message dispatch for Evochain -> OwnershipParachain messages.
pub type FromEvochainMessageDispatch =
	bridge_runtime_common::messages_xcm_extension::XcmBlobMessageDispatch<
		crate::OnOwnershipParachainBlobDispatcher,
		(),
	>;

/// Export XCM messages to be relayed to Evochain.
pub type ToEvochainBlobExporter =
	HaulBlobExporter<XcmBlobHaulerAdapter<ToEvochainXcmBlobHauler>, crate::EvochainNetwork, ()>;

/// To-Evochain XCM hauler.
pub struct ToEvochainXcmBlobHauler;

impl XcmBlobHauler for ToEvochainXcmBlobHauler {
	type MessageSender = pallet_bridge_messages::Pallet<Runtime, WithEvochainMessagesInstance>;

	fn xcm_lane() -> LaneId {
		LaneIdFromChainId::<Runtime, WithEvochainMessagesInstance>::get()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{EvochainGrandpaInstance, Runtime, WithEvochainMessagesInstance};
	use bridge_runtime_common::{
		assert_complete_bridge_types,
		integrity::{
			assert_complete_with_relay_chain_bridge_constants, check_message_lane_weights,
			AssertChainConstants, AssertCompleteBridgeConstants,
		},
	};

	#[test]
	fn ensure_evochain_message_lane_weights_are_correct() {
		check_message_lane_weights::<
			bp_ownership_parachain::OwnershipParachain,
			Runtime,
			WithEvochainMessagesInstance,
		>(
			bp_evochain::EXTRA_STORAGE_PROOF_SIZE,
			bp_ownership_parachain::MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX,
			bp_ownership_parachain::MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX,
			false,
		);
	}

	#[test]
	fn ensure_bridge_integrity() {
		assert_complete_bridge_types!(
			runtime: Runtime,
			with_bridged_chain_grandpa_instance: EvochainGrandpaInstance,
			with_bridged_chain_messages_instance: WithEvochainMessagesInstance,
			this_chain: bp_ownership_parachain::OwnershipParachain,
			bridged_chain: bp_evochain::Evochain,
		);

		assert_complete_with_relay_chain_bridge_constants::<
			Runtime,
			EvochainGrandpaInstance,
			WithEvochainMessagesInstance,
		>(AssertCompleteBridgeConstants {
			this_chain_constants: AssertChainConstants {
				block_length: bp_ownership_parachain::BlockLength::get(),
				block_weights: bp_ownership_parachain::BlockWeights::get(),
			},
		});
	}

	#[test]
	fn ownership_parachain_evochain_bridge_identifier_did_not_changed() {
		// there's nothing criminal if it is changed, but then thou need to fix it across
		// all deployments scripts, alerts and so on
		assert_eq!(
			*ToEvochainXcmBlobHauler::xcm_lane().as_ref(),
			hex_literal::hex!("6aa61bff567db6b5d5f0cb815ee6d8f5ac630e222a95700cb3d594134e3805de")
				.into(),
		);
	}
}
