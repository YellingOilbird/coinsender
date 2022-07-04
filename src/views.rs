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
        let vault = self.internal_get_user_vault(&account_id);
        VaultOutput::from(vault)
    }
}