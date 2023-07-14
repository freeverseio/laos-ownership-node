//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::Precompile;
use sp_std::marker::PhantomData;

/// Batch precompile.
#[derive(Debug, Clone)]
pub struct LivingAssetsPrecompile<Runtime>(PhantomData<Runtime>);

// No funds are transfered to the precompile address.
// Transfers will directly be made on the behalf of the user by the precompile.
impl<T> Precompile for LivingAssetsPrecompile<T>
where
	T: pallet_evm::Config,
{
	fn execute(handle: &mut impl fp_evm::PrecompileHandle) -> fp_evm::PrecompileResult {
		unimplemented!()
	}
}
