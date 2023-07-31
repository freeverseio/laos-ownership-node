use frame_support::dispatch::DispatchResult;

/// The `CollectionManager` trait provides an interface for managing collections in a
/// decentralized and non-fungible asset management system. This system allows for the creation of
/// collections, each of which can be owned by a unique `AccountId`.
///
/// A collection in this context can be thought of as a container for non-fungible assets.
/// Each collection has an associated `collection_id` which is a unique identifier for the collection
/// and can be used to retrieve the owner of the collection.
///
/// # Methods
///
/// - `owner_of_collection(collection_id: T::CollectionId) -> Option<AccountId>`: This method retrieves the owner
/// of a collection given its `collection_id`. If no collection exists with the provided `collection_id`,
/// the method returns `None`.
///
/// - `create_collection(who: AccountId) -> DispatchResult`: This method creates a
/// new collection and assigns ownership to the provided `AccountId`.
///
/// # Errors
///
/// [Error::CollectionIdOverflow]: Thrown if the collection counter overflows when creating a new collection.
pub trait CollectionManager<AccountId, CollectionId> {
	/// Get owner of collection
	fn owner_of_collection(collection_id: CollectionId) -> Option<AccountId>;

	/// Create collection
	fn create_collection(who: AccountId) -> DispatchResult;
}
