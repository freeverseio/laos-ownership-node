#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{generic, traits::{BlakeTwo256, IdentifyAccount, Verify}, MultiSignature};
use sp_core::H256;
	
/// An index to a block.
pub type BlockNumber = u32;

/// A hash of some data used by the chain.
pub type Hash = H256;

/// Hashing type
pub	type Hashing = BlakeTwo256;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// The type for storing how many extrinsics an account has signed.
pub type Nonce = u32;







