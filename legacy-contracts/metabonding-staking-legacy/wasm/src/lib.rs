// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           13
// Async Callback (empty):               1
// Total number of exported functions:  16

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    metabonding_staking_legacy
    (
        init => init
        upgrade => upgrade
        stakeLockedAsset => stake_locked_asset
        unstake => unstake
        unbond => unbond
        pause => pause_endpoint
        unpause => unpause_endpoint
        getStakedAmountForUser => get_staked_amount_for_user
        getUserEntry => get_user_entry
        getSnapshot => get_snapshot
        getLockedAssetTokenId => locked_asset_token_id
        getLockedAssetFactoryAddress => locked_asset_factory_address
        getTotalLockedAssetSupply => total_locked_asset_supply
        getUserList => user_list
        isPaused => is_paused
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
