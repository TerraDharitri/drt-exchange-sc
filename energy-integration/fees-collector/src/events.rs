dharitri_sc::imports!();

use common_types::Week;

#[dharitri_sc::module]
pub trait FeesCollectorEventsModule {
    fn emit_deposit_swap_fees_event(
        self,
        caller: ManagedAddress,
        current_week: Week,
        payment: DcdtTokenPayment<Self::Api>,
    ) {
        self.deposit_swap_fees_event(caller, current_week, payment);
    }

    #[event("deposit_swap_fees_event")]
    fn deposit_swap_fees_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] current_week: Week,
        #[indexed] payment: DcdtTokenPayment<Self::Api>,
    );
}
