// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           37
// Async Callback (empty):               1
// Total number of exported functions:  40

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    fees_collector
    (
        init => init
        upgrade => upgrade
        claimRewards => claim_rewards_endpoint
        claimBoostedRewards => claim_boosted_rewards
        addKnownContracts => add_known_contracts
        removeKnownContracts => remove_known_contracts
        addKnownTokens => add_known_tokens
        removeKnownTokens => remove_known_tokens
        getLockedTokenId => locked_token_id
        getAllTokens => get_all_tokens
        getAllKnownContracts => known_contracts
        getAllowExternalClaimRewards => allow_external_claim_rewards
        getLastActiveWeekForUser => get_last_active_week_for_user_view
        getUserEnergyForWeek => get_user_energy_for_week_view
        getLastGlobalUpdateWeek => last_global_update_week
        getTotalRewardsForWeek => total_rewards_for_week
        getTotalEnergyForWeek => total_energy_for_week
        getTotalLockedTokensForWeek => total_locked_tokens_for_week
        updateEnergyForUser => update_energy_for_user
        getCurrentClaimProgress => current_claim_progress
        depositSwapFees => deposit_swap_fees
        getAccumulatedFees => accumulated_fees
        setLockedTokensPerBlock => set_locked_tokens_per_block
        getLastLockedTokensAddWeek => last_locked_token_add_week
        getLockedTokensPerBlock => locked_tokens_per_block
        setLockingScAddress => set_locking_sc_address
        setLockEpochs => set_lock_epochs
        getLockingScAddress => locking_sc_address
        getLockEpochs => lock_epochs
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
        getCurrentWeek => get_current_week
        getFirstWeekStartEpoch => first_week_start_epoch
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
        addSCAddressToWhitelist => add_sc_address_to_whitelist
        removeSCAddressFromWhitelist => remove_sc_address_from_whitelist
        isSCAddressWhitelisted => is_sc_address_whitelisted
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
