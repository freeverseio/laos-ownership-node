use super::*;
use fp_evm::{ExitReason, Transfer};

pub struct MockHandle {
	pub input: Vec<u8>,
	pub gas_limit: Option<u64>,
	pub context: Context,
	pub is_static: bool,
	pub gas_used: u64,
	pub logs: Vec<Log>,
	pub code_address: H160,
}

impl MockHandle {
	pub fn new(input: Vec<u8>, gas_limit: Option<u64>, context: Context) -> Self {
		Self {
			input,
			gas_limit,
			context,
			is_static: false,
			gas_used: 0,
			logs: vec![],
			code_address: H160::zero(),
		}
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

	fn record_external_cost(&mut self, _: Option<u64>, _: Option<u64>) -> Result<(), ExitError> {
		Ok(())
	}

	fn refund_external_cost(&mut self, _: Option<u64>, _: Option<u64>) {}

	fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) -> Result<(), ExitError> {
		let log = Log { address, topics, data };
		self.logs.push(log);
		Ok(())
	}

	fn remaining_gas(&self) -> u64 {
		unimplemented!()
	}

	fn code_address(&self) -> H160 {
		self.code_address
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
