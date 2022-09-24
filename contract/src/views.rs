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
        result
    }

    pub fn get_user_vault(&self, account_id: AccountId) -> VaultOutput {
        let vault = self.internal_get_vault(&account_id);
        VaultOutput::from(vault)
    }

    /// Get user deposit by token contract. If calls without token_id returns NEAR deposit value
    pub fn get_user_deposit_by_token(&self, account_id: AccountId, token_id: Option<TokenContractId>) -> U128 {
        let vault = self.internal_get_vault_or_create(&account_id);
        vault.get_deposit_by_token(token_id)
    }

    /// Get active users list
    pub fn get_user_accounts(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<AccountId> {
        let accounts = &self.user_vaults;
        unordered_map_pagination(accounts, from_index, limit)
    } 

    pub fn get_donate_destinations(&self) -> Vec<AccountId> {
        self.donate_receivers.clone()
    }
}
