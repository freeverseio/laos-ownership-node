// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @dev The LivingAssets contract's address.
address constant LIVING_ASSETS_ADDRESS = 0x0000000000000000000000000000000000000101;

/// @dev The LivingAssets contract's instance.
LivingAssets constant LIVING_ASSETS_CONTRACT = LivingAssets(
    LIVING_ASSETS_ADDRESS
);

/// @author Freeverse team
/// @title Pallet LivingAssets Interface
/// @dev The interface through which solidity contracts will interact with pallet-living-assets
/// @custom:address 0x0000000000000000000000000000000000000101
interface LivingAssets {
    /// @dev Create collection
    /// @custom:selector ef8b6cd8
    ///
    /// @param collection_id The `collection_id` to be associated
    function create_collection(uint64 collection_id) external;

    /// @dev Get collection owner
    /// @custom:selector 8b0b20f4
    ///
    /// @param collection_id The `collection_id`
    function owner_of_collection(
        uint64 collection_id
    ) external returns (address);
}