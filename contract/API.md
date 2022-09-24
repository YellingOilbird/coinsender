## Fungible token methods
```js
viewMethods: ['storage_balance_of'],
changeMethods: ['storage_deposit', 'ft_transfer_call']
```

## Coinsender methods
#### VIEW

```js
viewMethods: ['multi_storage_deposit','send_from_balance_unsafe','send_from_balance','deposit_near'],
```

- Get whitelisted tokens
```rust
/// Fungible token are actually whitelisted into contract
WhitelistedToken {
    contract_id: AccountId,
    ticker: Option<String>,
    decimals: u8,
    icon: Option<String>
}
/// Returns list of tokens whitelisted in contract
fn get_whitelisted_tokens() -> [AccountId, WhitelistedToken];
```
- Get user account Vault
```rust
/// user Vault representation:
VaultOutput {
    // Human-readable amount of NEAR in user vault
    pub near_deposit: U128,
    // Human-readable amounts of FT in user vault
    pub token_deposits: Vec<(AccountId, U128)>,
    // user stats
    pub near_sends_num: u64,
    pub ft_sends_num: u64,
    pub tokens_used: Vec<String>,
    pub total_near_amount_sent: U128,
}
/// Returns user Vault representation
fn get_user_vault(account_id: AccountId) -> VaultOutput
```
- Get user application deposit (deposit into Contract)
```rust
/// When `token_id` was not set returns NEAR user deposit amount
/// When `token_id` is set returns given token contract user deposit amount
fn get_user_deposit_by_token(account_id: AccountId, token_id: Option<AccountId>) -> U128;
```
#### CHANGE

```js
changeMethods: ['multi_storage_deposit','send_from_balance_unsafe','send_from_balance','deposit_near'],
```
- Deposit NEAR
```rust
/// Deposits given NEAR amount to contract. If u need any other (fungible) token - go to send them through `ft_transfer_call`
/// Requires to attach 1 Yocto to call
/// Example 
///     - await contract.deposit_near({}, gas, amount)
fn deposit_near(amount: Balance);
```
- Multi - Storage deposit
```rust
/// Takes [] array of accounts and token_id
/// Requires to attach amount = X * 0.00125NEAR, 
/// where X is number of given accounts in function call args
/// For example:
/// 
/// let total_funded_required = bigInt(fundedAccountsChunks[i].length)
///     .multiply(bigInt("1250000000000000000000"))
///     .toString();
///            contract.multi_storage_deposit({
///                    accounts: fundedAccountsChunks[i],
///                    token_id: token_id
///                }, gas, total_funded_required
///            );
fn multi_storage_deposit(accounts: Vec<AccountId>, token_id: AccountId)
```

- Send
```rust
/// argument to send ( account amount ), like `account.near 3.2`
Operation: {
    account_id: AccountId,
    amount: U128
}
```
```rust
/// Takes [] array of account-amount and token_id
/// If `token_id` was not set - transfers NEAR from deposit to users
/// If `token_id` is set - transfers `token_id` from deposit to users
/// Requires to attach 1 Yocto to call, 
/// For example:
/// 
/// NEAR send
/// await contract.send_from_balance_unsafe({
///     accounts: chunk
/// }, gas, "1");
/// 
/// Any FT send
/// await contract.send_from_balance_unsafe({
///     accounts: chunk,
///     token_id: coin
/// }, gas, "1");
fn send_from_balance_unsafe(accounts: Vec<AccountId>, token_id: AccountId)
```

