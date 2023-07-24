#![allow(clippy::new_without_default)]

use pallet_evm::{
	IsPrecompileResult, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use polkadot_primitives::BlakeTwo256;
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_living_assets_ownership::LivingAssetsOwnershipPrecompile;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};

use crate::{AccountId, CollectionId};

pub struct FrontierPrecompiles<Runtime>(PhantomData<Runtime>);

impl<Runtime> FrontierPrecompiles<Runtime>
where
	Runtime: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> [H160; 7] {
		[hash(1), hash(2), hash(3), hash(4), hash(5), hash(1025), hash(1026)]
	}
}

type LivingAssetsPrecompile = LivingAssetsOwnershipPrecompile<
	pallet_evm::HashedAddressMapping<BlakeTwo256>,
	AccountId,
	CollectionId,
	pallet_living_assets_ownership::Pallet<crate::Runtime>,
>;

impl<Runtime> PrecompileSet for FrontierPrecompiles<Runtime>
where
	Runtime: pallet_evm::Config + pallet_living_assets_ownership::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			// Non-Frontier specific nor Ethereum precompiles :
			// a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
			a if a == hash(1026) => Some(LivingAssetsPrecompile::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer {
			is_precompile: Self::used_addresses().contains(&address),
			extra_cost: 0,
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::H160;
    use pallet_evm::PrecompileHandle;
    use sp_std::vec::Vec;

    struct MockPrecompileHandle {
        code_address: H160,
        input_data: Vec<u8>,
        // ... other fields if necessary
    }

    impl PrecompileHandle for MockPrecompileHandle {
        fn code_address(&self) -> H160 {
            self.code_address
        }

        // ... other required methods
    }

    #[test]
    fn test_execute_ecrecover() {
        let mock_handle = MockPrecompileHandle {
            code_address: hash(1),
            input_data: vec![...],  // fill with appropriate data
        };

        let frontier_precompiles = FrontierPrecompiles::new();
        let result = frontier_precompiles.execute(&mock_handle);

        assert!(result.is_some());
        // If you know what the result should be, you can compare it directly.
        // assert_eq!(result.unwrap(), ...);
    }

    // Additional tests can be written for other precompiles.
}