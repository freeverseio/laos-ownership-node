use super::mock::*;

#[test]
fn check_precompiled() {
	new_test_ext().execute_with(|| {
        // PrecompilesValue::is_precompile(hash(1), 0);
    })
}
