use crate::mock::new_test_ext;

#[test]
fn it_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(2 + 2, 4);
	});
}
