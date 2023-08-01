use super::{hash, mock::*, FrontierPrecompiles};
use pallet_evm::{IsPrecompileResult, PrecompileSet};
use sp_core::H160;

fn is_precompile(address: H160) -> bool {
	let p = FrontierPrecompiles::<Runtime>::new();
	let result = p.is_precompile(address, 0);
	if let IsPrecompileResult::Answer { is_precompile, extra_cost } = result {
		is_precompile
	} else {
		panic!("Unexpected result variant");
	}
}

#[test]
fn null_address_is_not_precompile() {
    assert!(!is_precompile(H160::zero()));
}

#[test]
fn ethrerum_precompiled_reserved_addresses_are_precompiled() {
    assert!(is_precompile(hash(1)));
    assert!(is_precompile(hash(2)));
    assert!(is_precompile(hash(3)));
    assert!(is_precompile(hash(4)));
    assert!(is_precompile(hash(5)));
}
