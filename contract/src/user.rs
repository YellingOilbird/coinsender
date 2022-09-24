use crate::*;

/// User Account vault
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
    // amount of NEAR in user vault
    pub near_deposit: Balance,
    // amounts of FT in user vault
    pub token_deposits: UnorderedMap<AccountId, Balance>,
    //stats
    // amounts of FT sended from vault
    pub total_ft_amount_sent: UnorderedMap<AccountId, Balance>,
    near_sends_num: u64,
    spended_storage: Balance,
    ft_sends_num: u64,
    tokens_used: Vec<String>,
    total_near_amount_sent: Balance,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VaultOutput {
    // Human-readable amount of NEAR in user vault
    pub near_deposit: U128,
    // Human-readable amounts of FT in user vault
    pub token_deposits: Vec<(AccountId, U128)>,
    //stats
    // Human-readable amounts of FT sended from vault
    pub total_ft_amount_sent: Vec<(AccountId, U128)>,
    pub near_sends_num: u64,
    pub ft_sends_num: u64,
    pub spended_storage: U128,
    pub tokens_used: Vec<String>,
    pub total_near_amount_sent: U128,
}

impl From<Vault> for VaultOutput {
    fn from(v: Vault) -> Self {
        Self { 
            near_deposit: v.near_deposit.into(), 
            token_deposits: make_view_token_deposits(v.token_deposits), 
            near_sends_num: v.near_sends_num,
            spended_storage: v.spended_storage.into(),
            ft_sends_num: v.ft_sends_num, 
            tokens_used: v.tokens_used, 
            total_near_amount_sent: v.total_near_amount_sent.into(),
            total_ft_amount_sent: make_view_token_deposits(v.total_ft_amount_sent)
        }
    }
}

/// Versioned Vault
#[derive(BorshSerialize, BorshDeserialize)]
pub enum VVault {
    V1(Vault),
    Current(Vault),
}
impl VVault {
    pub fn into(self) -> Vault {
        match self {
            VVault::V1(_v) => unimplemented!(),
            VVault::Current(v) => v,
        }
    }
}
impl From<Vault> for VVault {
    fn from(v: Vault) -> Self {
        VVault::Current(v)
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum UpdateVaultAction {
    AfterNewSend,
    AfterNotSuccessSend,
    AfterDeposit,
    AfterWithdraw,
    AfterMultiStorageDeposit
}

impl Vault {
    fn new() -> Self {
        Self { 
            near_deposit: 0, 
            token_deposits: UnorderedMap::new(StorageKey::TokenDeposits),
            total_ft_amount_sent: UnorderedMap::new(StorageKey::TokensSended),
            spended_storage: 0, 
            near_sends_num: 0,
            ft_sends_num: 0, 
            tokens_used: vec![], 
            total_near_amount_sent: 0, 
        }
    }

    pub fn get_previous_ft_amount_sended(&self, token_id: &TokenContractId) -> Balance {
        if let Some(w_balance) = self.total_ft_amount_sent.get(token_id) {
            w_balance
        } else {
            NO_DEPOSIT
        }
    } 

    pub fn is_empty(&self) -> bool {
        let has_ft_deposits = self.token_deposits.iter()
            .any(|(_, w_balance)| {
                w_balance != NO_DEPOSIT
            });
        let has_near_deposit = {
            self.near_deposit > 0
        };

        if !has_ft_deposits && !has_near_deposit {
            true
        } else {
            false
        }
        
    }

    // Gets user deposit by token contract. If calls without token_id returns NEAR deposit value
    pub fn get_deposit_by_token(&self, token_id: Option<TokenContractId>) -> U128 {
        if let Some(unwrapped_token_id) = token_id {
            match self.token_deposits.get(&unwrapped_token_id) {
                Some(deposit) => {
                    U128(deposit)
                }
                None => {
                    U128(0)
                }
            }
        } else {
            U128(self.near_deposit)
        }
    }
}

impl Contract {
    pub (crate) fn internal_get_vault(&self, account_id: &AccountId) -> Vault {
        self.user_vaults.get(account_id)
            .map(|v| v.into())
            .expect(ERR_UNKNOWN_USER)
    }

    pub(crate) fn internal_get_vault_or_create(&self, account_id: &AccountId) -> Vault {
        if let Some(vault) = self.user_vaults.get(account_id) {
            vault.into()
        } else {
            Vault::new()
        }
    }

    pub(crate) fn internal_update_user_vault(
        &mut self,
        action: UpdateVaultAction, 
        vault: &mut Vault,
        previous_balance: Option<Balance>,
        token_id: Option<TokenContractId>,
        tokens_used: Option<Balance>
    ) {

        let tokens_used = tokens_used.unwrap_or(NO_DEPOSIT);

        match action {
            // After success send from balance update
            UpdateVaultAction::AfterNewSend => {
                // FT
                if let Some(unwrapped_token_id) = token_id {
                    let ticker = self.get_token_ticker(unwrapped_token_id.clone());
                    let new_balance = previous_balance.unwrap() - tokens_used;
                    if tokens_used > 0 {
                        let mut prev_amount = vault.get_previous_ft_amount_sended(&unwrapped_token_id);
                        prev_amount += tokens_used;

                        vault.total_ft_amount_sent.insert(&unwrapped_token_id, &prev_amount);
                        vault.ft_sends_num += 1;
                    };
                    if !vault.tokens_used.contains(&ticker) {
                        vault.tokens_used.push(ticker.clone())
                    };
                    vault.token_deposits.insert(&unwrapped_token_id, &new_balance);
                // NEAR
                } else {
                    vault.near_sends_num += 1;
                    vault.total_near_amount_sent += tokens_used;
                    vault.near_deposit -= tokens_used;
                }
            },
            // After not success send from balance update (only for safety-transfer, not for unsafe)
            UpdateVaultAction::AfterNotSuccessSend => {
                // FT
                if let Some(unwrapped_token_id) = token_id {
                    let new_balance = previous_balance.unwrap() + tokens_used;
                    vault.token_deposits.insert(&unwrapped_token_id, &new_balance);
                // NEAR
                } else {
                    vault.near_deposit += tokens_used;
                }
            },
            // After deposit update
            UpdateVaultAction::AfterDeposit => {
                // FT
                if let Some(unwrapped_token_id) = token_id {
                    let new_balance = previous_balance.unwrap() + tokens_used;
                    vault.token_deposits.insert(&unwrapped_token_id, &new_balance);
                // NEAR
                } else {
                    vault.near_deposit += tokens_used;
                }
            },
            UpdateVaultAction::AfterWithdraw => {
                // FT
                if let Some(unwrapped_token_id) = token_id {
                    vault.token_deposits.insert(&unwrapped_token_id, &NO_DEPOSIT);
                // NEAR
                } else {
                    vault.near_deposit = NO_DEPOSIT;
                }
            },
            UpdateVaultAction::AfterMultiStorageDeposit => {
                vault.spended_storage += tokens_used
            }
        }
    }

    pub(crate) fn internal_deposit_ft(&mut self, account_id: AccountId, deposit_amount: U128, token_id: TokenContractId) {
        let attached_tokens: u128 = deposit_amount.0; 
        let mut vault = self.internal_get_vault_or_create(&account_id);
        let previous_balance: u128 = vault
            .get_deposit_by_token(Some(token_id.clone()))
            .0;

        self.internal_update_user_vault(
            UpdateVaultAction::AfterDeposit, 
            &mut vault, 
            Some(previous_balance), 
            Some(token_id), 
            Some(attached_tokens)
        );
        // insert updated one
        self.user_vaults.insert(&account_id, &vault.into());
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn deposit_near(&mut self)  {
        let attached_tokens: u128 = env::attached_deposit(); 
        let account_id = env::predecessor_account_id();
        let mut vault = self.internal_get_vault_or_create(&account_id);
        let previous_balance: u128 = vault
            .get_deposit_by_token(None)
            .0;

        // update info about user deposit in MULTISENDER
        self.internal_update_user_vault(
            UpdateVaultAction::AfterDeposit, 
            &mut vault, 
            Some(previous_balance),
            None, 
            Some(attached_tokens)
        );
        // insert updated one
        self.user_vaults.insert(&account_id, &vault.into());
    }

    // withdraw all from multisender
    #[payable]
    pub fn withdraw_all(&mut self, token_id: Option<TokenContractId>) {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let mut vault = self.internal_get_vault(&account_id);
        let deposit: u128 = vault
            .get_deposit_by_token(token_id.clone())
            .0;
        // FT
        if let Some(unwrapped_token_id) = token_id {
            assert!(self.user_vaults.get(&account_id).is_some(), "{}", ERR_UNKNOWN_USER);
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
                &mut vault, 
                None,
                Some(unwrapped_token_id.clone()), 
                None
            );
        // NEAR
        } else {
            assert!(
                deposit > NO_DEPOSIT,
                "{}", ERR_NOTHING_TO_WITHDRAW
            );
            Promise::new(account_id.clone()).transfer(deposit);
            self.internal_update_user_vault(
                UpdateVaultAction::AfterWithdraw, 
                &mut vault, 
                None,
                None, 
                None
            );
        }
        // insert updated one
        self.user_vaults.insert(&account_id, &vault.into());
    }


    #[payable]
    pub fn remove_vault(&mut self) {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let vault: Vault = self.internal_get_vault(&account_id);
        if vault.is_empty() {
            self.user_vaults.remove(&account_id);
            log!("Removed vault for user @{} ", account_id);
        }
    }
    
    //multisender transfer callback
    #[private]
    pub fn on_transfer_from_balance(&mut self, recipient_id: AccountId, amount_sent: U128, sender_id: AccountId, token_id: Option<TokenContractId>) {
        let transfer_succeeded = is_promise_success();
        let mut vault = self.internal_get_vault(&recipient_id);
        let previous_balance: u128 = vault.get_deposit_by_token(token_id.clone()).0;

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
                    &mut vault, 
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
                    &mut vault, 
                    None, 
                    None, 
                    Some(amount_sent.0)
                );
            }
            // insert updated one
            self.user_vaults.insert(&recipient_id, &vault.into());
        }
    }
}
