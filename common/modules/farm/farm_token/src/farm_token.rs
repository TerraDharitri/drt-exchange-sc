#![no_std]

dharitri_sc::imports!();
dharitri_sc::derive_imports!();

use common_structs::Nonce;

#[dharitri_sc::module]
pub trait FarmTokenModule:
    permissions_module::PermissionsModule
    + dharitri_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[payable("REWA")]
    #[endpoint(registerFarmToken)]
    fn register_farm_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        self.require_caller_has_owner_or_admin_permissions();

        let payment_amount = self.call_value().rewa_value().clone_value();
        self.farm_token().issue_and_set_all_roles(
            DcdtTokenType::Meta,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    fn burn_farm_tokens_from_payments(&self, payments: &ManagedVec<DcdtTokenPayment<Self::Api>>) {
        let mut total_amount = BigUint::zero();
        for entry in payments.iter() {
            total_amount += &entry.amount;
            self.send()
                .dcdt_local_burn(&entry.token_identifier, entry.token_nonce, &entry.amount);
        }

        self.farm_token_supply().update(|x| *x -= total_amount);
    }

    fn mint_farm_tokens<T: TopEncode>(
        &self,
        token_id: TokenIdentifier,
        amount: BigUint,
        attributes: &T,
    ) -> DcdtTokenPayment<Self::Api> {
        let new_nonce = self
            .send()
            .dcdt_nft_create_compact(&token_id, &amount, attributes);
        self.farm_token_supply().update(|x| *x += &amount);

        DcdtTokenPayment::new(token_id, new_nonce, amount)
    }

    fn burn_farm_tokens(&self, token_id: &TokenIdentifier, nonce: Nonce, amount: &BigUint) {
        self.send().dcdt_local_burn(token_id, nonce, amount);
        self.farm_token_supply().update(|x| *x -= amount);
    }

    fn burn_farm_token_payment(&self, payment: &DcdtTokenPayment<Self::Api>) {
        self.burn_farm_tokens(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    fn get_farm_token_attributes<T: TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
    ) -> T {
        let token_info = self.blockchain().get_dcdt_token_data(
            &self.blockchain().get_sc_address(),
            token_id,
            token_nonce,
        );

        token_info.decode_attributes()
    }

    #[view(getFarmTokenId)]
    #[storage_mapper("farm_token_id")]
    fn farm_token(&self) -> NonFungibleTokenMapper;

    #[view(getFarmTokenSupply)]
    #[storage_mapper("farm_token_supply")]
    fn farm_token_supply(&self) -> SingleValueMapper<BigUint>;
}
