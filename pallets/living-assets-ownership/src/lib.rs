#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use sp_core::H160;

mod functions;
pub mod traits;

#[frame_support::pallet]
pub mod pallet {
	use crate::functions::convert_asset_id_to_owner;

	use super::*;
	use frame_support::pallet_prelude::{OptionQuery, ValueQuery, *};
	use frame_system::pallet_prelude::*;
	use sp_core::{H160, U256};

	/// Collection id type
	/// TODO: use 256 bits
	pub type CollectionId = u64;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/// Mapping from collection id to owner
	#[pallet::storage]
	#[pallet::getter(fn owner_of_collection)]
	pub(super) type OwnerOfCollection<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, T::AccountId, OptionQuery>;

	/// Collection counter
	#[pallet::storage]
	#[pallet::getter(fn collection_counter)]
	pub(super) type CollectionCounter<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	/// Pallet events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: CollectionId, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Collection already exists
		CollectionAlreadyExists,
		/// Collection id overflow
		CollectionIdOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO set proper weight
		pub fn create_collection(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match Self::do_create_collection(who) {
				Ok(_) => Ok(()),
				Err(err) => Err(err.into()),
			}
		}
	}

	impl<T: Config> traits::CollectionManager<T::AccountId> for Pallet<T> {
		fn owner_of_collection(collection_id: CollectionId) -> Option<T::AccountId> {
			OwnerOfCollection::<T>::get(collection_id)
		}

		fn create_collection(owner: T::AccountId) -> Result<CollectionId, &'static str> {
			Self::do_create_collection(owner)
		}
	}

	impl<T: Config> traits::Erc721 for Pallet<T> {
		fn owner_of(collection_id: CollectionId, asset_id: U256) -> Result<H160, &'static str> {
			match OwnerOfCollection::<T>::get(collection_id) {
				Some(_) => Ok(convert_asset_id_to_owner(asset_id)),
				None => Err("Collection does not exist"),
			}
		}
	}
}

/// `ASSET_PRECOMPILE_ADDRESS_PREFIX` is a predefined prefix used to identify collection addresses.
///
/// All addresses that start with this prefix are considered as collection addresses.
/// Since `CollectionId` is represented as a `u64`, it leaves these bits free to be
/// utilized for such a prefix.
///
/// Usage of this prefix provides a consistent and recognizable pattern for distinguishing
/// collection addresses from other types of addresses in the system.
pub const ASSET_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[0xff; 12];

#[derive(Debug, PartialEq)]
pub enum CollectionError {
	InvalidPrefix,
	SliceLengthMismatch,
}

pub fn collection_id_to_address(collection_id: CollectionId) -> H160 {
	let mut bytes = [0u8; 20];
	bytes[12..20].copy_from_slice(&collection_id.to_be_bytes());
	for (i, byte) in ASSET_PRECOMPILE_ADDRESS_PREFIX.iter().enumerate() {
		bytes[i] = *byte;
	}
	H160(bytes)
}

pub fn address_to_collection_id(address: H160) -> Result<CollectionId, CollectionError> {
	if &address.0[0..12] != ASSET_PRECOMPILE_ADDRESS_PREFIX {
		return Err(CollectionError::InvalidPrefix);
	}
	let id_bytes: [u8; 8] = address.0[12..20].try_into().unwrap();
	Ok(CollectionId::from_be_bytes(id_bytes))
}

pub fn is_collection_address(address: H160) -> bool {
	&address.to_fixed_bytes()[0..12] == ASSET_PRECOMPILE_ADDRESS_PREFIX
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
