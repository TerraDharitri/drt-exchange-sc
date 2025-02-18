// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           27
// Async Callback:                       1
// Total number of exported functions:  30

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    price_discovery
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        withdraw => withdraw
        redeem => redeem
        getCurrentPrice => calculate_price
        getMinLaunchedTokenPrice => min_launched_token_price
        getPricePrecision => price_precision
        getLaunchedTokenId => launched_token_id
        getAcceptedTokenId => accepted_token_id
        getLaunchedTokenBalance => launched_token_balance
        getAcceptedTokenBalance => accepted_token_balance
        getStartBlock => start_block
        getEndBlock => end_block
        setLockingScAddress => set_locking_sc_address
        setUnlockEpoch => set_unlock_epoch
        getLockingScAddress => locking_sc_address
        getUnlockEpoch => unlock_epoch
        getCurrentPhase => get_current_phase
        getNoLimitPhaseDurationBlocks => no_limit_phase_duration_blocks
        getLinearPenaltyPhaseDurationBlocks => linear_penalty_phase_duration_blocks
        getFixedPenaltyPhaseDurationBlocks => fixed_penalty_phase_duration_blocks
        getPenaltyMinPercentage => penalty_min_percentage
        getPenaltyMaxPercentage => penalty_max_percentage
        getFixedPenaltyPercentage => fixed_penalty_percentage
        issueRedeemToken => issue_redeem_token
        createInitialRedeemTokens => create_initial_redeem_tokens
        getRedeemTokenId => redeem_token
        getRedeemTokenTotalCirculatingSupply => redeem_token_total_circulating_supply
    )
}

dharitri_sc_wasm_adapter::async_callback! { price_discovery }
