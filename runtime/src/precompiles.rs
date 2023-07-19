//! Define precompiles for the runtime

use pallet_evm::{
	ExitRevert, IsPrecompileResult, Precompile, PrecompileFailure, PrecompileHandle,
	PrecompileResult, PrecompileSet,
};
use pallet_evm_living_assets_ownership::LivingAssetsOwnershipPrecompile;
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_ed25519::Ed25519Verify;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

/// LAOS network precompiles
#[derive(Debug, Default, Clone, Copy)]
pub struct LaosPrecompiles<R>(PhantomData<R>);

impl<R> LaosPrecompiles<R> {
	pub fn new() -> Self {
		Self(Default::default())
	}

	/// Return all addresses that contain precompiles. This can be used to populate dummy code
	/// under the precompile.
	pub fn used_addresses() -> impl Iterator<Item = H160> {
		sp_std::vec![1, 2, 3, 4, 5, 6, 7, 8, 1024, 1025, 1026, 1027, 20481]
			.into_iter()
			.map(hash)
	}
}

impl<R> PrecompileSet for LaosPrecompiles<R>
where
	LivingAssetsOwnershipPrecompile<R>: Precompile,
	R: pallet_evm::Config + pallet_living_assets_ownership::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		let address = handle.code_address();
		if let IsPrecompileResult::Answer { is_precompile, .. } =
			self.is_precompile(address, u64::MAX)
		{
			if is_precompile && address > hash(9) && handle.context().address != address {
				return Some(Err(PrecompileFailure::Revert {
					exit_status: ExitRevert::Reverted,
					output: b"cannot be called with DELEGATECALL or CALLCODE".to_vec(),
				}))
			}
		}
		match address {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			a if a == hash(6) => Some(Bn128Add::execute(handle)),
			a if a == hash(7) => Some(Bn128Mul::execute(handle)),
			a if a == hash(8) => Some(Bn128Pairing::execute(handle)),
			a if a == hash(9) => Some(Blake2F::execute(handle)),
			// nor Ethereum precompiles :
			a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			a if a == hash(1026) => Some(ECRecoverPublicKey::execute(handle)),
			a if a == hash(1027) => Some(Ed25519Verify::execute(handle)),
			// LAOS precompiles
			a if a == hash(20481) => Some(LivingAssetsOwnershipPrecompile::<R>::execute(handle)),
			// Default
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, remaining_gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer {
			is_precompile: Self::used_addresses().any(|x| x == address),
			extra_cost: 0,
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
