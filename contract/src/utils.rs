use crate::*;
use near_sdk::ONE_NEAR;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId, Balance, BorshStorageKey, Gas, PromiseResult, json_types::U128};

/*
storage constants

STORAGE_PRICE_PER_BYTE   - Price per 1 byte of storage from mainnet genesis config.
STORAGE_DEPOSIT          - Price per storage registration into Fungible Token contract.
*/
pub const STORAGE_PRICE_PER_BYTE: Balance = 10_000_000_000_000_000_000;
pub const STORAGE_DEPOSIT:Balance = 125 * STORAGE_PRICE_PER_BYTE;
/// gas constants
pub const ONE_YOCTO: Balance = 1;
pub const NO_DEPOSIT: Balance = 0;
pub const CALLBACK_GAS: Gas = Gas(Gas::ONE_TERA.0 * 5);

/// State errors
#[allow(dead_code)]
pub const ERR_EXCEEDED_OF_PREPAID_GAS: &str = "Too low gas attached";
/// Multisender errors
pub const ERR_UNKNOWN_USER: &str = "User don't have any deposited tokens on Multisender balance";
pub const ERR_TOKEN_NOT_WHITELISTED: &str = "Cannot find this token in whitelisted. You must whitelist this one before deposit";
pub const ERR_NOTHING_TO_WITHDRAW: &str = "No tokens on multisender balance to withdraw!. Check your balances";
pub const ERR_NOT_ALLOWED: &str = "Owner's method";
pub const ERR_OWNER_NOT_SET: &str = "Contract doesn't have an owner right now";
/// Callback errors
pub const ERR_FAILED_DEPOSIT_TRANSFER: &str = "Something wrong with transfer call from you to token contract. Check your balances";
pub const ERR_FAILED_PROMISE: &str = "Promise failed! Expected single result of callback!";

pub const TREASURY_LIMIT:Balance = 100 * ONE_NEAR;

pub (crate) type TokenContractId = AccountId;

//StorageKey implementation for Default prefixes in Multisender contract
#[derive(BorshSerialize, BorshStorageKey)]
pub (crate) enum StorageKey {
    WhitelistedTokens,
    UserDeposits,
    TokenDeposits,
    TokensSended
}
//Transfer calls msg instructions
pub (crate) enum TransferInstruction {
    Unknown,
    Default,
    Deposit,
}

//Configure transfer callback actions via msg
impl From<String> for TransferInstruction {
    fn from(item: String) -> Self {
        match &item[..] {
            "deposit" => TransferInstruction::Deposit,
            ""        => TransferInstruction::Default,
            _         => TransferInstruction::Unknown,
        }
    }
}

// Chunk from receiver's accounts from front-end side of Multisender
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Operation {
    pub account_id: AccountId,
    pub amount: U128,
}

#[allow(dead_code)]
pub (crate) fn is_promise_success() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Call"
    );
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}
#[allow(dead_code)]
pub (crate) fn yocto_ft(yocto_amount: Balance, decimals: u8) -> u128 {
    (yocto_amount + (5 * 10u128.pow((decimals - 1u8).into()))) / 10u128.pow(decimals.into())
}
#[allow(dead_code)]
pub (crate) fn one_ft(decimals: u8) -> Balance {
    //1 FT is MIN_DEPOSIT
    10u128.pow((decimals).into())
}

pub (crate) fn unordered_map_pagination<K, V>(
    m: &UnorderedMap<K, V>,
    from_index: Option<u64>,
    limit: Option<u64>,
) -> Vec<K>
    where
        K: BorshSerialize + BorshDeserialize,
        V: BorshSerialize + BorshDeserialize,
{
    let keys = m.keys_as_vector();
    let from_index = from_index.unwrap_or(0);
    let limit = limit.unwrap_or(keys.len());
    (from_index..std::cmp::min(keys.len(), from_index + limit))
        .map(|index| keys.get(index).unwrap())
        .collect()
}

pub (crate) fn make_view_token_deposits(map: UnorderedMap<AccountId, Balance>) -> Vec<(AccountId, U128)> {
    map.to_vec()
        .iter()
        .map(|(acc, bal)|{
            (acc.clone(), U128(*bal))
        })
        .collect()
}