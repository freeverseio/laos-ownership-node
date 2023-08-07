use super::*;
use helpers::*;
use pallet_living_assets_ownership::CollectionId;
use sp_core::{H160, U256};

type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

#[test]
fn check_selectors() {
	assert_eq!(Action::OwnerOf as u32, 0x6352211E);
	assert_eq!(Action::TockenURI as u32, 0xC87B56DD);
}

#[test]
fn owner_of_unexistent_should_return_null_address() {
	impl_precompile_mock_simple!(Mock, None);

	let owner_of_1234 = "6352211e0000000000000000000000000000000000000000000000000000000000000004";
	let mut handle = create_mock_handle_from_input(owner_of_1234);
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	assert_eq!(
		hex::encode(result.unwrap().output),
		"0000000000000000000000000000000000000000000000000000000012345678"
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
	use evm::{Context, ExitError, ExitReason, Transfer};
	use fp_evm::{Log, PrecompileHandle};
	use sp_core::{H160, H256};

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
				fn owner_of(collectio_id: CollectionId, asset_id: U256) -> Option<H160> {
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

	/// Create a mock handle for testing precompiled contracts.
	///
	/// This function takes an input string representing the data to be sent to the precompiled contract
	/// and a cost value, returning a `MockHandle` that can be used for testing.
	///
	/// # Arguments
	///
	/// * `input` - The input data as a hexadecimal string.
	/// * `cost` - A cost value as u64.
	/// * `value` - The amount of coins transferred as u64.
	///
	/// # Example
	///
	/// ```
	/// let handle = create_mock_handle("68656c6c6f", 0, 0);
	/// ```
	pub fn create_mock_handle(input: &str, cost: u64, value: u64, caller: H160) -> MockHandle {
		let i: Vec<u8> = hex::decode(input).expect("invalid input");

		let context: Context =
			Context { address: Default::default(), caller, apparent_value: From::from(value) };

		MockHandle::new(i, Some(cost), context)
	}

	/// Create a mock handle for testing precompiled contracts without a specific cost or value.
	///
	/// This function takes an input string representing the data to be sent to the precompiled contract
	/// and returns a `MockHandle` that can be used for testing.
	///
	/// # Arguments
	///
	/// * `input` - The input data as a hexadecimal string.
	///
	/// # Example
	///
	/// ```
	/// let handle = create_mock_handle_from_input("68656c6c6f");
	/// ```
	pub fn create_mock_handle_from_input(input: &str) -> MockHandle {
		create_mock_handle(input, 0, 0, H160::zero())
	}

	pub struct MockHandle {
		pub input: Vec<u8>,
		pub gas_limit: Option<u64>,
		pub context: Context,
		pub is_static: bool,
		pub gas_used: u64,
		pub logs: Vec<Log>,
	}

	impl MockHandle {
		pub fn new(input: Vec<u8>, gas_limit: Option<u64>, context: Context) -> Self {
			Self { input, gas_limit, context, is_static: false, gas_used: 0, logs: vec![] }
		}
	}

	impl PrecompileHandle for MockHandle {
		/// Perform subcall in provided context.
		/// Precompile specifies in which context the subcall is executed.
		fn call(
			&mut self,
			_: H160,
			_: Option<Transfer>,
			_: Vec<u8>,
			_: Option<u64>,
			_: bool,
			_: &Context,
		) -> (ExitReason, Vec<u8>) {
			unimplemented!()
		}

		fn record_cost(&mut self, cost: u64) -> Result<(), ExitError> {
			self.gas_used += cost;
			Ok(())
		}

		fn record_external_cost(
			&mut self,
			_: Option<u64>,
			_: Option<u64>,
		) -> Result<(), ExitError> {
			Ok(())
		}

		fn refund_external_cost(&mut self, _: Option<u64>, _: Option<u64>) {}

		fn log(
			&mut self,
			address: H160,
			topics: Vec<H256>,
			data: Vec<u8>,
		) -> Result<(), ExitError> {
			let log = Log { address, topics, data };
			self.logs.push(log);
			Ok(())
		}

		fn remaining_gas(&self) -> u64 {
			unimplemented!()
		}

		fn code_address(&self) -> H160 {
			unimplemented!()
		}

		fn input(&self) -> &[u8] {
			&self.input
		}

		fn context(&self) -> &Context {
			&self.context
		}

		fn is_static(&self) -> bool {
			self.is_static
		}

		fn gas_limit(&self) -> Option<u64> {
			self.gas_limit
		}
	}
}
