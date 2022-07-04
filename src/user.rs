use crate::{*, tokens::ft_contract};

pub (crate) type WeightedBalance = (Balance, u8);
pub const E24_DECIMALS:u8 = 24;

/// User Account vault
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Vault {
    // amount of NEAR in user vault
    near_deposit: Balance,
    // amounts of FT in user vault
    token_deposits: UnorderedMap<AccountId, WeightedBalance>,
    // stats
    near_sends_num: u64,
    ft_sends_num: u64,
    tokens_used: Vec<String>,
    total_near_amount_sent: Balance,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VaultOutput {
    // Human-readable amount of NEAR in user vault
    pub near_deposit: Balance,
    // Human-readable amounts of FT in user vault
    pub token_deposits: Vec<(AccountId, Balance)>,
    // stats
    pub near_sends_num: u64,
    pub ft_sends_num: u64,
    pub tokens_used: Vec<String>,
    pub total_near_amount_sent: Balance,
}

impl From<Vault> for VaultOutput {
    fn from(v: Vault) -> Self {
        Self { 
            near_deposit: yocto_ft(v.near_deposit, E24_DECIMALS), 
            token_deposits: v.make_view_token_deposits(), 
            near_sends_num: v.near_sends_num,
            ft_sends_num: v.ft_sends_num, 
            tokens_used: v.tokens_used, 
            total_near_amount_sent: yocto_ft(v.total_near_amount_sent, E24_DECIMALS), 
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VVault {
    Current(Vault)
}

impl From<VVault> for Vault {
    fn from(v: VVault) -> Self {
        match v {
            VVault::Current(c) => c
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum UpdateVaultAction {
    AfterNewSend,
    AfterNotSuccessSend,
    AfterDeposit,
    AfterWithdraw
}

impl Vault {
    fn new() -> Self {
        Self { 
            near_deposit: 0, 
            token_deposits: UnorderedMap::new(StorageKey::TokenDeposits), 
            near_sends_num: 0,
            ft_sends_num: 0, 
            tokens_used: vec![], 
            total_near_amount_sent: 0, 
        }
    }
    fn make_view_token_deposits(&self) -> Vec<(AccountId, Balance)> {
        let token_deposits = &self.token_deposits;
        let mut result:Vec<(AccountId, Balance)> = vec![];
        for token_id in token_deposits.keys() {
            let (deposit, decimals) = token_deposits.get(&token_id).unwrap_or((0,E24_DECIMALS));
            result.push((token_id, yocto_ft(deposit, decimals)));
        };
        result
    }
}

impl Contract {
    pub(crate) fn internal_get_user_vault(&self, account_id: &AccountId) -> Vault {
        if let Some(vault) = self.user_vaults.get(account_id) {
            vault.into()
        } else {
            Vault::new()
        }
    }
    pub(crate) fn internal_update_user_vault(
        &mut self,
        action: UpdateVaultAction, 
        sender_id: AccountId,
        previous_balance: Option<Balance>,
        token_id: Option<TokenContractId>,
        tokens_used: Option<Balance>
    ) {
        // get vault
        let mut vault = self.internal_get_user_vault(&sender_id);
        let tokens_used = tokens_used.unwrap_or(NO_DEPOSIT);

        // After success send from balance update
        if action == UpdateVaultAction::AfterNewSend {
            // FT
            if let Some(unwrapped_token_id) = token_id {
                // TODO : use this to print human-readable views in VaultOutput
                let token_data = self.tokens.get(&unwrapped_token_id.clone()).unwrap();
                let decimals = token_data.decimals;
                let ticker = token_data.ticker.unwrap();
                let new_balance = previous_balance.unwrap() - tokens_used;

                vault.ft_sends_num += 1;
                if !vault.tokens_used.contains(&ticker) {
                    vault.tokens_used.push(ticker.clone())
                };
                vault.token_deposits.insert(&unwrapped_token_id, &(new_balance, decimals));
            // NEAR
            } else {
                vault.near_sends_num += 1;
                vault.total_near_amount_sent += tokens_used;
                vault.near_deposit -= tokens_used;
            }
        // After not success send from balance update (only for safety-transfer, not for unsafe)
        } else if action == UpdateVaultAction::AfterNotSuccessSend {
            if let Some(unwrapped_token_id) = token_id {
                let token_data = self.tokens.get(&unwrapped_token_id.clone()).unwrap();
                let decimals = token_data.decimals;
                let new_balance = previous_balance.unwrap() + tokens_used;

                vault.ft_sends_num -= 1;
                vault.token_deposits.insert(&unwrapped_token_id, &(new_balance, decimals));
            } else {
                vault.near_sends_num -= 1;
                vault.total_near_amount_sent -= tokens_used;
                vault.near_deposit += tokens_used;
            }
        // After deposit update
        // FT
        } else if action == UpdateVaultAction::AfterDeposit {
            if let Some(unwrapped_token_id) = token_id {
                let token_data = self.tokens.get(&unwrapped_token_id.clone()).unwrap();
                let decimals = token_data.decimals;
                let new_balance = previous_balance.unwrap() + tokens_used;

                vault.token_deposits.insert(&unwrapped_token_id, &(new_balance, decimals));
            } else {
                vault.near_deposit += tokens_used;
            }
        // NEAR
        } else if action == UpdateVaultAction::AfterWithdraw {
            if let Some(unwrapped_token_id) = token_id {
                vault.token_deposits.insert(&unwrapped_token_id, &(NO_DEPOSIT, E24_DECIMALS));
            } else {
                vault.near_deposit = NO_DEPOSIT;
            }
        }
        // insert updated one
        self.user_vaults.insert(&sender_id, &VVault::Current(vault));
    }
    pub(crate) fn internal_deposit_ft(&mut self, account_id: AccountId, deposit_amount: U128, token_id: TokenContractId) {
        let attached_tokens: u128 = deposit_amount.0; 
        let previous_balance: u128 = self.get_deposit_by_token(account_id.clone(), Some(token_id.clone())).into();
        self.internal_update_user_vault(
            UpdateVaultAction::AfterDeposit, 
            account_id, 
            Some(previous_balance), 
            Some(token_id), 
            Some(attached_tokens)
        );
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn deposit_near(&mut self)  {
        let attached_tokens: u128 = env::attached_deposit(); 
        let account_id = env::predecessor_account_id();

        // update info about user deposit in MULTISENDER
        self.internal_update_user_vault(
            UpdateVaultAction::AfterDeposit, 
            account_id, 
            None,
            None, 
            Some(attached_tokens)
        );
    }

    // withdraw all from multisender
    #[payable]
    pub fn withdraw_all(&mut self, account_id: AccountId, token_id: Option<TokenContractId>) {
        assert_one_yocto();
        let deposit: u128 = self.get_deposit_by_token(account_id.clone(), token_id.clone()).into();
        // FT
        if let Some(unwrapped_token_id) = token_id {
            assert!(self.user_vaults.contains_key(&account_id), "{}", ERR_UNKNOWN_USER);
            assert!(
                deposit > NO_DEPOSIT,
                "{}", ERR_NOTHING_TO_WITHDRAW
            );

            ft_contract::ext(unwrapped_token_id.clone())
            .with_attached_deposit(ONE_YOCTO)
            .with_static_gas(CALLBACK_GAS)
            .ft_transfer(account_id.clone(), deposit.into(), None);

            self.internal_update_user_vault(
                UpdateVaultAction::AfterWithdraw, 
                account_id, 
                None,
                None, 
                None
            );
        // NEAR
        } else {
            Promise::new(account_id.clone()).transfer(deposit);
            self.internal_update_user_vault(
                UpdateVaultAction::AfterWithdraw, 
                account_id, 
                None,
                None, 
                None
            );
        }
    }
    
    //multisender transfer callback
    #[private]
    pub fn on_transfer_from_balance(&mut self, recipient_id: AccountId, amount_sent: U128, sender_id: AccountId, token_id: Option<TokenContractId>) {
        let transfer_succeeded = is_promise_success();
        let previous_balance: u128 = self.get_deposit_by_token(recipient_id.clone(), token_id.clone()).0;

        if !transfer_succeeded {
            // FT
            if let Some(unwrapped_token_id) = token_id {
                let token_decimals = self.get_token_decimals(unwrapped_token_id.clone());
                let token_ticker = self.get_token_ticker(unwrapped_token_id.clone());
                log!("Transaction from @{} to @{} failed. {} ${} (~{} {}) kept on the app deposit",
                    sender_id,
                    recipient_id,
                    amount_sent.0,
                    token_ticker.clone(),
                    yocto_ft(amount_sent.0, token_decimals),
                    token_ticker.clone()
                );

                self.internal_update_user_vault(
                    UpdateVaultAction::AfterNotSuccessSend, 
                    sender_id, 
                    Some(previous_balance), 
                    Some(unwrapped_token_id), 
                    Some(amount_sent.0)
                );
            // NEAR
            } else {
                log!("Transaction from @{} to @{} failed. {} yNEAR  (~{} NEAR) kept on the app deposit",
                    sender_id,
                    recipient_id,
                    amount_sent.0,
                    yocto_ft(amount_sent.0, 24),
                );
                self.internal_update_user_vault(
                    UpdateVaultAction::AfterNotSuccessSend, 
                    sender_id, 
                    None, 
                    None, 
                    Some(amount_sent.0)
                );
            }
        }
    }
    #[private]
    // Gets user deposit by token contract. If calls without token_id returns NEAR deposit value
    pub fn get_deposit_by_token(&self, account_id: AccountId, token_id: Option<TokenContractId>) -> U128 {
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

