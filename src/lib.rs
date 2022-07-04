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
        //FT
        if let Some(unwrapped_token_id) = token_id.clone() {
            assert!(self.is_token_whitelisted(unwrapped_token_id.clone()), "{}", ERR_TOKEN_NOT_WHITELISTED);
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
            assert!(self.is_token_whitelisted(unwrapped_token_id.clone()), "{}", ERR_TOKEN_NOT_WHITELISTED);
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