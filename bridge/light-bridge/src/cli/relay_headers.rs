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

use async_trait::async_trait;
use relay_substrate_client::{AccountIdOf, AccountKeyPairOf};
use sp_core::Pair;
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

use crate::bridges::ownership_parachain_evochain::{
	evochain_headers_to_ownership_parachain::EvochainToOwnershipParachainCliBridge,
	millau_headers_to_ownership_parachain::MillauToOwnershipParachainCliBridge,
};
use relay_substrate_client::Client;
use relay_utils::metrics::{GlobalMetrics, StandaloneMetric};
use substrate_relay_helper::finality::SubstrateFinalitySyncPipeline;

use crate::cli::{bridge::*, chain_schema::*, PrometheusParams};

/// Start headers relayer process.
#[derive(StructOpt)]
pub struct RelayHeaders {
	/// A bridge instance to relay headers for.
	#[structopt(possible_values = RelayHeadersBridge::VARIANTS, case_insensitive = true)]
	bridge: RelayHeadersBridge,
	/// If passed, only mandatory headers (headers that are changing the GRANDPA authorities set)
	/// are relayed.
	#[structopt(long)]
	only_mandatory_headers: bool,
	#[structopt(flatten)]
	source: SourceConnectionParams,
	#[structopt(flatten)]
	target: TargetConnectionParams,
	#[structopt(flatten)]
	target_sign: TargetSigningParams,
	#[structopt(flatten)]
	prometheus_params: PrometheusParams,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
/// Headers relay bridge.
pub enum RelayHeadersBridge {
	EvochainToOwnershipParachain,
	MillauToOwnershipParachain,
}

#[async_trait]
trait HeadersRelayer: RelayToRelayHeadersCliBridge
where
	AccountIdOf<Self::Target>: From<<AccountKeyPairOf<Self::Target> as Pair>::Public>,
{
	/// Relay headers.
	async fn relay_headers(data: RelayHeaders) -> anyhow::Result<()> {
		let source_client = data.source.into_client::<Self::Source>().await?;
		let target_client = data.target.into_client::<Self::Target>().await?;
		let target_transactions_mortality = data.target_sign.target_transactions_mortality;
		let target_sign = data.target_sign.to_keypair::<Self::Target>()?;

		let metrics_params: relay_utils::metrics::MetricsParams =
			data.prometheus_params.into_metrics_params()?;
		GlobalMetrics::new()?.register_and_spawn(&metrics_params.registry)?;

		let target_transactions_params = substrate_relay_helper::TransactionParams {
			signer: target_sign,
			mortality: target_transactions_mortality,
		};
		Self::Finality::start_relay_guards(
			&target_client,
			&target_transactions_params,
			target_client.can_start_version_guard(),
		)
		.await?;

		substrate_relay_helper::finality::run::<Self::Finality>(
			source_client,
			target_client,
			data.only_mandatory_headers,
			target_transactions_params,
			metrics_params,
		)
		.await
	}
}

impl HeadersRelayer for EvochainToOwnershipParachainCliBridge {}
impl HeadersRelayer for MillauToOwnershipParachainCliBridge {}

impl RelayHeaders {
	/// Run the command.
	pub async fn run(self) -> anyhow::Result<()> {
		match self.bridge {
			RelayHeadersBridge::EvochainToOwnershipParachain => {
				EvochainToOwnershipParachainCliBridge::relay_headers(self)
			},
			RelayHeadersBridge::MillauToOwnershipParachain => {
				MillauToOwnershipParachainCliBridge::relay_headers(self)
			},
		}
		.await
	}
}
