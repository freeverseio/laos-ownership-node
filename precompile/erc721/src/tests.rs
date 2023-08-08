use core::str::FromStr;

use super::*;
use evm::{ExitError, ExitRevert};
use fp_evm::PrecompileFailure;
use pallet_living_assets_ownership::CollectionId;
use precompile_utils::testing::create_mock_handle_from_input;
use sp_core::{H160, U256};

type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

#[test]
fn check_selectors() {
	assert_eq!(Action::OwnerOf as u32, 0x6352211E);
	assert_eq!(Action::TockenURI as u32, 0xC87B56DD);
}

#[test]
fn owner_of_asset_should_return_an_address() {
	impl_precompile_mock_simple!(
		Mock,
		Ok(H160::from_str("ff00000000000000000000000000000012345678").unwrap())
	);

	let owner_of_asset_4 =
		hex::decode("6352211e0000000000000000000000000000000000000000000000000000000000000004")
			.unwrap();
	let mut handle = create_mock_handle_from_input(owner_of_asset_4);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	assert_eq!(
		hex::encode(result.unwrap().output),
		"000000000000000000000000ff00000000000000000000000000000012345678",
	);
}

#[test]
fn if_mock_fails_should_return_the_error() {
	impl_precompile_mock_simple!(Mock, Err("spaghetti error"));

	let owner_of_asset_4 =
		hex::decode("6352211e0000000000000000000000000000000000000000000000000000000000000004")
			.unwrap();
	let mut handle = create_mock_handle_from_input(owner_of_asset_4);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(
		result.unwrap_err(),
		PrecompileFailure::Error {
			exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed("spaghetti error"))
		}
	);
}

#[test]
fn invalid_contract_address_should_error() {
	impl_precompile_mock_simple!(Mock, Ok(H160::zero()));

	let mut handle = create_mock_handle_from_input(Vec::new());
	handle.code_address = H160::zero();
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(
		result.unwrap_err(),
		PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: vec![
				116, 114, 105, 101, 100, 32, 116, 111, 32, 112, 97, 114, 115, 101, 32, 115, 101,
				108, 101, 99, 116, 111, 114, 32, 111, 117, 116, 32, 111, 102, 32, 98, 111, 117,
				110, 100, 115
			]
		}
	);
}

// #[test]
// fn owner_of() {
// 	impl_precompile_mock!(Mock, |collection_id, asset_id| {
// 		assert_eq!(collection_id, 0);
// 		assert_eq!(asset_id, U256::from("0x1234"));
// 		Some(H160::from(2))
// 	});

// 	let owner_of_1234 = "6352211e0000000000000000000000000000000000000000000000000000000000001234";
// 	let mut handle = create_mock_handle_from_input(owner_of_1234);
// 	let result = Mock::execute(&mut handle);
// 	assert!(result.is_ok());
// 	assert_eq!(result.unwrap().output, H160::zero().encode());
// }
mod helpers {
	/// Macro to define a precompile mock with custom closures for testing.
	///
	/// This macro creates mock implementations of the `Erc721` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// You can define custom closures for the create_collection and owner_of_collection functions.
	///
	/// # Arguments
	///
	/// * `$name`: An identifier to name the precompile mock type.
	/// * `$create_collection_result`: A closure that takes `collection_id` and `who` and returns a `DispatchResult`.
	/// * `$owner_of_collection_result`: A closure that takes `collection_id` and returns an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock!(
	///     MyMock,
	///     |who| { Ok(0) },
	///     |collection_id| { Some(H160::zero()) }
	/// );
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock {
		($name:ident, $owner_of_collection:expr) => {
			struct Erc721Mock;

			impl pallet_living_assets_ownership::traits::Erc721 for Erc721Mock {
				fn owner_of(
					collectio_id: CollectionId,
					asset_id: U256,
				) -> Result<AccountId, &'static str> {
					($owner_of_collection)(collectio_id, asset_id)
				}
			}

			type $name = Erc721Precompile<AddressMapping, AccountId, Erc721Mock>;
		};
	}

	/// Macro to define a precompile mock for testing.
	///
	/// This macro creates mock implementations of the `Erc721` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// The mock type is named `Mock`, and the implementation uses the provided expressions.
	///
	/// # Arguments
	///
	/// * `$create_collection_result`: An expression that evaluates to a `DispatchResult`.
	/// * `$owner_of_collection_result`: An expression that evaluates to an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(0), Some(H160::zero()));
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock_simple {
		($name:ident, $owner_of_collection:expr) => {
			impl_precompile_mock!($name, |_asset_id, _collection_id| { $owner_of_collection });
		};
	}
}
