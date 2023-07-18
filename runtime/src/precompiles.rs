//! Defines the precompiles available on the runtime.

use pallet_evm_living_assets_ownership::LivingAssetsOwnershipPrecompile;
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use precompile_utils::precompile_set::*;

/// Explicit precompiles related to Ethereum.
type EthereumPrecompilesChecks = (AcceptDelegateCall, CallableByContract, CallableByPrecompile);

#[precompile_utils::precompile_name_from_address]
type LaosPrecompileSet<Runtime> = (
	// Ethereum precompiles.
	// We allow DELEGATECALL to stay compliant with Ethereum behavior.
	PrecompileAt<AddressU64<1>, ECRecover, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<2>, Sha256, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<3>, Ripemd160, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<4>, Identity, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<5>, Modexp, EthereumPrecompilesChecks>,
	// Non-Frontier specific nor Ethereum precompiles :
	PrecompileAt<AddressU64<6>, Sha3FIPS256, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<7>, ECRecoverPublicKey, EthereumPrecompilesChecks>,
	// LAOS precompiles
	PrecompileAt<AddressU64<8>, LivingAssetsOwnershipPrecompile<Runtime>, (CallableByContract, CallableByPrecompile)>,
);

pub type LaosPrecompiles<Runtime> = PrecompileSetBuilder<Runtime, (
	PrecompilesInRangeInclusive<(AddressU64<1>, AddressU64<4095>), LaosPrecompilesAt<Runtime>>,
)>;
