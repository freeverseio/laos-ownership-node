use crate::LivingAssetsOwnershipPrecompile;
use pallet_evm::Precompile;
use pallet_evm_test_vector_support::MockHandle;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dddd() {
        LivingAssetsOwnershipPrecompile::<bool>::new();
        <LivingAssetsOwnershipPrecompile<bool> as Precompile>::execute(&mut 0);
    }
}