use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_whitelisted_tokens(&self) -> Vec<(AccountId, WhitelistedToken)> {
        let tokens = &self.tokens;
        let mut result:Vec<(AccountId, WhitelistedToken)> = vec![];
        for token_id in tokens.keys() {
            let token_data = tokens.get(&token_id).expect(ERR_TOKEN_NOT_WHITELISTED);
            result.push((token_id, token_data));
        };
        log!("on_wl_token");
        result
    }
    pub fn get_user_vault(&self, account_id: AccountId) -> VaultOutput {
        let vault = self.internal_get_user_vault(&account_id);
        VaultOutput::from(vault)
    }
    // Gets user deposit by token contract. If calls without token_id returns NEAR deposit value
    // Non private view method
    pub fn get_user_deposit_by_token(&self, token_id: Option<TokenContractId>) -> U128 {
        let account_id: AccountId = env::predecessor_account_id();
        let vault = self.internal_get_user_vault(&account_id);
        if let Some(unwrapped_token_id) = token_id {
            match vault.token_deposits.get(&unwrapped_token_id) {
                Some((deposit, _)) => {
                    deposit.into()
                }
                None => {
                    0.into()
                }
            }
        } else {
            vault.near_deposit.into()
        }
    }
}