use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

/// User Account vault
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WhitelistedToken {
    pub contract_id: TokenContractId,
    pub ticker: Option<String>,
    pub decimals: u8,
}

impl WhitelistedToken {
    fn new(contract_id: TokenContractId, ticker: Option<String>, decimals: u8) -> Self {
        Self {
            contract_id, 
            ticker, 
            decimals 
        }
    }
}

/// Define the methods we'll use on the other contract
#[ext_contract(ft_contract)]
pub trait FungibleToken {
    fn storage_deposit(&self, account_id: AccountId);
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalance {
    pub total: U128,
    pub available: U128,
}

impl Contract {
    pub (crate) fn is_token_whitelisted(&self, token_id: TokenContractId) -> bool {
        match self.tokens.get(&token_id) {
            Some(_) => true,
            None => false,
        }
    }
    pub (crate) fn get_token_decimals(&self, token_id: TokenContractId) -> u8 {
        let token_data = self.tokens
            .get(&token_id)
            .expect(ERR_TOKEN_NOT_WHITELISTED);
        token_data.decimals
    }
    pub (crate) fn get_token_ticker(&self, token_id: TokenContractId) -> String {
        let token_data = self.tokens
            .get(&token_id)
            .expect(ERR_TOKEN_NOT_WHITELISTED);
        token_data.ticker.unwrap()
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn whitelist_token(&mut self, token_id: AccountId) {

        assert!(self.assert_owner_or_self(), "{}", ERR_NOT_ALLOWED);
        assert_one_yocto();
        //storage deposit for our contract for have ability to receive deposits in this token
        ft_contract::ext(token_id.clone())
            .with_attached_deposit(STORAGE_DEPOSIT)
            .with_static_gas(CALLBACK_GAS)
                .storage_deposit(env::predecessor_account_id());

        ft_contract::ext(token_id.clone())
            .with_static_gas(CALLBACK_GAS)
            .ft_metadata()
                .then(Self::ext(env::current_account_id())
                .with_static_gas(CALLBACK_GAS)
                .on_ft_metadata(token_id)
            );
    }
    //register multiple accounts to TOKEN_CONTRACT. Because of gas limit it may be only less then 50 accounts
    #[payable]
    pub fn multi_storage_deposit(&mut self, accounts: Vec<AccountId>, token_id: TokenContractId) {
        
        assert!(self.is_token_whitelisted(token_id.clone()),"{}", ERR_TOKEN_NOT_WHITELISTED);

        let total_accounts = accounts.len();
        assert!(total_accounts <= 50,"{}", ERR_TOO_MANY_ACCOUNTS);

        // deposit requested for storage_deposit for 1 account into FT contract
        let storage_bond: u128 = 125 * STORAGE_PRICE_PER_BYTE;

        // deposit requested for storage_deposit for all accounts into FT contract
        let total_storage_bond: u128 = storage_bond * total_accounts as u128;

        assert!(
            env::attached_deposit() >= total_storage_bond,
            "ERR_SMALL_DEPOSIT: YOU NEED {} yN MORE FOR THIS FUNCTION_CALL", (total_storage_bond - env::attached_deposit())
        );

        for account in accounts {
            ft_contract::ext(token_id.clone())
                .with_attached_deposit(STORAGE_DEPOSIT)
                .with_static_gas(CALLBACK_GAS)
                    .storage_deposit(account.clone());
            log!("Register or check @{} into {} token storage", account, self.get_token_ticker(token_id.clone()))
        }
    }

    #[private]
    pub fn on_ft_metadata(&mut self, token_id: TokenContractId) {

        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => panic!("{}", ERR_FAILED_PROMISE),
            PromiseResult::Successful(result) => {
                let ft_metadata = near_sdk::serde_json::from_slice::<FungibleTokenMetadata>(&result).unwrap();
                let token_data = WhitelistedToken::new(
                    token_id.clone(),
                    Some(ft_metadata.symbol),
                    ft_metadata.decimals
                );
                self.tokens.insert(&token_id, &token_data);
                log!("token ${:?} successfully whitelisted", token_data.ticker.unwrap());
            }
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        //token contract which calls this function
        let token_id = env::predecessor_account_id();
        assert!(self.is_token_whitelisted(token_id.clone()), "{}", ERR_TOKEN_NOT_WHITELISTED);
    
        match TransferInstruction::from(msg) {
    
            TransferInstruction::Deposit => {
                log!("in deposit from @{} with token_contract_id: {} amount {} ", sender_id.clone(), token_id, amount.0);
                self.internal_deposit_ft(sender_id, amount, token_id);
                PromiseOrValue::Value(U128(0))
            },
            TransferInstruction::Default => unreachable!(),
            TransferInstruction::Unknown => {
                log!(ERR_FAILED_DEPOSIT_TRANSFER);
                PromiseOrValue::Value(amount)
            }
    
        }
    }
}
