#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::{ecdsa};

#[cfg(feature = "std")]
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, sp_core::RuntimeDebug, TypeInfo)]
pub struct EthereumSignature(ecdsa::Signature);

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::Pair;

    #[test]
    fn test_ecdsa_signature() {
        // Generate a random key pair
		let pair = ecdsa::Pair::from_string("//Alice///password", None).unwrap();

        // Sign the message
        let signature = pair.sign(b"Hello, World!");

        // Check that the signature is not empty (all zeros)
        assert_eq!(hex::encode(signature), "1165d9a3bd097685d04fb54b9c0e312aba2617ebeee84664055b789e454b29826acf86d7d71592f28bf6de1b94bb20f1458eb3a385b621d4ee9c0130415a92fd00");
    }
}