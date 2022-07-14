use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, env, log, ext_contract, near_bindgen, AccountId, PromiseResult, Balance, PromiseOrValue, Promise};
use near_sdk::serde::{Deserialize, Serialize};

use utils::*;
use user::*;
use tokens::*;

mod user;
mod utils;
mod tokens;
mod views;
mod upgrade;
mod web4;
mod web4helper;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: Option<AccountId>,
    user_vaults: LookupMap<AccountId, VVault>,
    tokens: UnorderedMap<TokenContractId, WhitelistedToken>,
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            owner_id: None,
            user_vaults: LookupMap::new(StorageKey::UserDeposits),
            tokens: UnorderedMap::new(StorageKey::WhitelistedTokens),
        }
    }
}


#[near_bindgen]
impl Contract {
    //register multiple accounts to TOKEN_CONTRACT. Because of gas limit it may be only less then 50 accounts
    #[payable]
    pub fn multi_storage_deposit(&mut self, accounts: Vec<AccountId>, token_id: TokenContractId) {

        let total_accounts = accounts.len();
        assert!(total_accounts <= 50, "ERR_TOO_MANY_ACCOUNTS!");
        assert!(self.is_token_whitelisted(token_id.clone()),"{}", ERR_TOKEN_NOT_WHITELISTED);

        //deposit requested for storage_deposit for 1 account into FT contract
        let storage_bond: u128 = 125 * STORAGE_PRICE_PER_BYTE;

        //deposit requested for storage_deposit for all accounts into FT contract
        let total_storage_bond: u128 = storage_bond * total_accounts as u128;

        assert!(
            env::attached_deposit() >= total_storage_bond,
            "ERR_SMALL_DEPOSIT: YOU NEED {} yN MORE FOR THIS FUNCTION_CALL", (total_storage_bond - env::attached_deposit())
        );

        for account in accounts {
            
            ft_contract::ext(token_id.clone())
                .with_static_gas(CALLBACK_GAS)
                .with_attached_deposit(storage_bond)
                .storage_deposit(account.clone());

            log!("Register storage for account @{}", account);
        }
    }
    //multisender transfer from deposit
    #[payable]
    pub fn send_from_balance(&mut self, accounts: Vec<Operation>, token_id: Option<TokenContractId>) {
        assert_one_yocto();
        //TODO - accounts chunks
        let account_id = env::predecessor_account_id();
        assert!(self.user_vaults.contains_key(&account_id), "{}", ERR_UNKNOWN_USER);
        let user_balance = self.get_deposit_by_token(account_id.clone(), token_id.clone()).0;
        let mut total = 0;


        for account in &accounts {
            let amount: Balance = account.amount.0;
            total += amount;
        }

        assert!(
            total <= user_balance,
            "Not enough deposited tokens to send (Supplied: {}. Demand: {})",
            user_balance,
            total
        );

        let mut total_sended:u128 = 0;
        // FT
        if let Some(unwrapped_token_id) = token_id.clone() {
            assert!(self.is_token_whitelisted(unwrapped_token_id.clone()),"{}", ERR_TOKEN_NOT_WHITELISTED);
            for account in accounts {
                ft_contract::ext(unwrapped_token_id.clone())
                .with_attached_deposit(ONE_YOCTO)
                .with_static_gas(CALLBACK_GAS)
                .ft_transfer(account.account_id.clone(), account.amount, None)
                .then(Self::ext(env::current_account_id())
                    .with_static_gas(CALLBACK_GAS)
                    .on_transfer_from_balance(account.account_id.clone(), account.amount, account.account_id.clone(), token_id.clone())
                );
                total_sended += account.amount.0;
            }
            self.internal_update_user_vault(UpdateVaultAction::AfterNewSend, account_id, Some(user_balance), token_id, Some(total_sended));
        // NEAR
        } else {
            for account in accounts {
                Promise::new(account.account_id.clone())
                    .transfer(account.amount.0)
                    .then(Self::ext(env::current_account_id())
                        .with_static_gas(CALLBACK_GAS)
                        .on_transfer_from_balance(account.account_id.clone(), account.amount, account.account_id.clone(), None)
                );
                total_sended += account.amount.0;
            }
            self.internal_update_user_vault(UpdateVaultAction::AfterNewSend,account_id, None, None, Some(total_sended));
        }
    }
    // Multisend from balance without callbacks - better gas efficient, but not usable for 2FA accs.
    // Allows 30 operations per transaction. But ChunkSize = 25 is reccomended (setting in App.js button event)
    #[payable]
    pub fn send_from_balance_unsafe(&mut self, accounts: Vec<Operation>, token_id: Option<TokenContractId>) {
        assert_one_yocto();
        //TODO - add vault update!!!
        let account_id = env::predecessor_account_id();
        assert!(self.user_vaults.contains_key(&account_id), "{}", ERR_UNKNOWN_USER);
        let user_balance = self.get_deposit_by_token(account_id.clone(), token_id.clone()).0;
        let mut total = 0;

        for account in &accounts {
            let amount: Balance = account.amount.0;
            total += amount;
        }
        assert!(
            total <= user_balance,
            "Not enough deposited tokens to send (Supplied: {}. Demand: {})",
            user_balance,
            total
        );
        let mut total_sended:u128 = 0;
        //FT
        if let Some(unwrapped_token_id) = token_id.clone() {
            assert!(self.is_token_whitelisted(unwrapped_token_id.clone()),"{}", ERR_TOKEN_NOT_WHITELISTED);
            for account in accounts {
                ft_contract::ext(unwrapped_token_id.clone())
                .with_attached_deposit(ONE_YOCTO)
                .with_static_gas(CALLBACK_GAS)
                    .ft_transfer(account.account_id.clone(), account.amount, None);

                total_sended += account.amount.0;
            }
            self.internal_update_user_vault(UpdateVaultAction::AfterNewSend, account_id, Some(user_balance), token_id, Some(total_sended));
        // NEAR
        } else {
            for account in accounts {
                Promise::new(account.account_id.clone())
                    .transfer(account.amount.0);
                
                total_sended += account.amount.0;
            }
            self.internal_update_user_vault(UpdateVaultAction::AfterNewSend, account_id, None, None, Some(total_sended));
        }
    }
}