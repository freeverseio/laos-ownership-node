use super::{hash, mock::*, FrontierPrecompiles};
use pallet_evm::{IsPrecompileResult, PrecompileSet};

#[test]
fn check_precompiled() {
	new_test_ext().execute_with(|| {
		let p = FrontierPrecompiles::<Runtime>::new();
		let result = p.is_precompile(hash(4), 0);
		if let IsPrecompileResult::Answer { is_precompile, extra_cost } = result {
			assert_eq!(is_precompile, true);
			assert_eq!(extra_cost, 0);
		} else {
			panic!("Unexpected result variant");
		}
	})
}
