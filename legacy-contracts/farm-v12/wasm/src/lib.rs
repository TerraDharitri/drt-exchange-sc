// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           36
// Async Callback (empty):               1
// Total number of exported functions:  38

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    farm_v12
    (
        init => init
        acceptFee => accept_fee
        calculateRewardsForGivenPosition => calculate_rewards_for_given_position
        end_produce_rewards_as_owner => end_produce_rewards_as_owner
        exitFarm => exit_farm
        getBurnedTokenAmount => burned_tokens
        getCurrentBlockFee => current_block_fee_storage
        getDivisionSafetyConstant => division_safety_constant
        getFarmTokenId => farm_token_id
        getFarmTokenSupply => get_farm_token_supply
        getFarmingTokenId => farming_token_id
        getFarmingTokenReserve => farming_token_reserve
        getLastErrorMessage => last_error_message
        getLastRewardBlockNonce => last_reward_block_nonce
        getLockedAssetFactoryManagedAddress => locked_asset_factory_address
        getLockedRewardAprMuliplier => locked_rewards_apr_multiplier
        getMinimumFarmingEpoch => minimum_farming_epoch
        getOwner => owner
        getPairContractManagedAddress => pair_contract_address
        getPenaltyPercent => penalty_percent
        getPerBlockRewardAmount => per_block_reward_amount
        getRewardPerShare => reward_per_share
        getRewardReserve => reward_reserve
        getRewardTokenId => reward_token_id
        getRouterManagedAddress => router_address
        getState => state
        getTransferExecGasLimit => transfer_exec_gas_limit
        getUndistributedFees => undistributed_fee_storage
        pause => pause
        resume => resume
        setPerBlockRewardAmount => set_per_block_reward_amount
        setTransferRoleFarmToken => set_transfer_role_farm_token
        set_locked_rewards_apr_multiplier => set_locked_rewards_apr_multiplier
        set_minimum_farming_epochs => set_minimum_farming_epochs
        set_penalty_percent => set_penalty_percent
        set_transfer_exec_gas_limit => set_transfer_exec_gas_limit
        start_produce_rewards => start_produce_rewards
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
