use crate::LivingAssetsOwnershipPrecompile;
use pallet_evm::Precompile;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn livingassetownershipprocompile_has_to_satisfy_precompile_trait() {
        fn requires_precompile_trait(_precompile: Precompile) {
            // If we can pass an instance of LivingAssetsOwnershipPrecompile to this function,
            // it means that LivingAssetsOwnershipPrecompile implements the Precompile trait.
        }
        let precompile = LivingAssetsOwnershipPrecompile::<bool>::new();
        requires_precompile_trait(precompile);
    }
}
