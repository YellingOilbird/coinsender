![](./contract/common/assets/coinsender_logo.png)

# COINSENDER 💸 
#### see user vault web4 example on https://coinsender.testnet.page/processing/finality/rmlsnk.testnet where u can paste your user account to see stats
#### see TODO file for progress
### links:
https://coinsender.testnet.page - testnet  
https://coinsender.near.page - mainnet

## Realised features:
### contract side
- ### *Rust smart-contract included:*
- Send functions which takes many accounts to transfer NEAR
- Send functions which takes many accounts to transfer FT
- Deposit with token options for more user-friendly interactions
- Whitelist token functionality with included registration in token storage for contract
- Multi-storage registration for many users per one transaction
- ```Vault``` which implements simple stats and update actions for checking occupation and reward users by activity in the NEAR future
- Contract works without initialization but owner can set after by call ```set_owner()``` from ```contract_id```

- ### *WEB4*
- web4 implemented
- Front-end side creates GET links which retrieved by contract web4 module and processing to show many views
- User balance comes directly from Rust

- ### *JS/TS(Front-end)
- Custom function for convertation choosed token to yocto amounts based on ```formatNearAmount``` from ```near-api-js```
- 90-s styled design from https://nostalgic-css.github.io/NES.css/[NES-CSS]
- Sequence of state changes using only ```localStorage```
- When token whitelisted by owner he actually comes to index page as pretty-view with icon and also as SELECT option

#### Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
    `cargo build --target wasm32-unknown-unknown --release`

#### Contract deploy

```bash
near dev-deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm

near deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm --accountId coinsender.testnet
```
mainnet
```
export NEAR_ENV=mainnet
near deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm --accountId coinsender.near
```
#### envs

```shell
# testnet
export CONTRACT_ID=coinsender.testnet
export AURORA=aurora.fakes.testnet
export WETH=weth.fakes.testnet
export PARAS=paras.fakes.testnet
export WNEAR=wrap.testnet
export DAI=dai.fakes.testnet
export USDC=usdc.fakes.testnet
export USN=usdn.testnet
export CHEDDAR=token-v3.cheddar.testnet
export USDT=usdt.fakes.testnet
export LNC=lnc.factory.tokenhub.testnet
export REF=ref.fakes.testnet
# mainnet
export CONTRACT_ID=coinsender.near
export USDT=dac17f958d2ee523a2206206994597c13d831ec7.factory.bridge.near
export DAI=6b175474e89094c44da98b954eedeac495271d0f.factory.bridge.near
export WNEAR=wrap.near
export USN=usn.near
export USDC=a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.factory.bridge.near
export REF=token.v2.ref-finance.near
export SKYWARD=token.skyward.near	
export OCT=f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near
export PARAS=token.paras.near
export stNEAR=meta-pool.near	
export WETH=c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2.factory.bridge.near
export CHEDDAR=token.cheddar.near	
export ETH=aurora
export AURORA=aaaaaa20d9e0e2461697782ef11675f668207961.factory.bridge.near
# accounts
export OWNER=
export USER_ACCOUNT=
export USER_ACCOUNT2=
export GAS=300000000000000
#E24 decimals NEP-141 tokens
export HUNDRED_TOKENS=100000000000000000000000000
export ONE_TOKEN=1000000000000000000000000
export FIVE_TOKENS=5000000000000000000000000
export TEN_TOKENS=10000000000000000000000000
#E18 decimals NEP-141 tokens
export HUNDRED_TOKENS=100000000000000000000
export ONE_TOKEN=1000000000000000000
export FIVE_TOKENS=5000000000000000000
export TEN_TOKENS=10000000000000000000
```
```sh
#set owner
near call $CONTRACT_ID set_owner '{"new_owner":"'$OWNER'"}' --accountId $CONTRACT_ID --gas $GAS --depositYocto 1
```

#### tokens
```shell
# private
near call $CONTRACT_ID whitelist_token '{"token_id": "'$stNEAR'"}' --accountId $CONTRACT_ID --depositYocto 1 --gas $GAS #or can be called from $OWNER
near call $CONTRACT_ID remove_token '{"token_id": "'$REF'"}' --accountId $CONTRACT_ID  --gas $GAS     #or can be called from $OWNER

near call $CONTRACT_ID get_whitelisted_tokens '' --accountId $CONTRACT_ID
```
#### vault
```shell
near call $CONTRACT_ID deposit_near '' --accountId $USER_ACCOUNT --amount 2 --gas $GAS
# E18 decimals use accurate!
near call guacharo.testnet ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$TEN_TOKENS'", "msg":"deposit"}' --accountId rmlsnk.testnet --depositYocto 1 --gas $GAS
# E18 decimals use accurate!
near call $LNC ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$TEN_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT2 --depositYocto 1 --gas $GAS
# Withdraw all ( in case NEAR withdraw no args for function call)
near call $CONTRACT_ID withdraw_all '{"token_id":"'$LNC'"}' --accountId $USER_ACCOUNT2 --depositYocto 1 --gas $GAS # FT
near call $CONTRACT_ID withdraw_all '' --accountId $USER_ACCOUNT2 --depositYocto 1 --gas $GAS # NEAR

#view
near call $CONTRACT_ID get_deposit_by_token '{"account_id": "'$USER_ACCOUNT'", "token_id":"'$REF'"}' --accountId $USER_ACCOUNT
near call $CONTRACT_ID get_user_vault '{"account_id": "'rmlsnk.testnet'"}' --accountId rmlsnk.testnet
near call $CONTRACT_ID get_user_accounts '{}' --accountId $USER_ACCOUNT
```
#### SEND !!!
```shell
near call $CONTRACT_ID send_from_balance_unsafe '{
    "accounts": [
        {
          "account_id": "participant_1.testnet",
          "amount": "'$ONE_TOKEN'"
        },
        {
          "account_id": "participant_2.testnet",
          "amount": "'$ONE_TOKEN'"
        },
        {
          "account_id": "participant_3.testnet",
          "amount": "'$ONE_TOKEN'"
        },
        {
          "account_id": "participant_4.testnet",
          "amount": "'$ONE_TOKEN'"
        }
    ],
    "token_id": "'$REF'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS

near call $CONTRACT_ID send_from_balance '{
    "accounts": [
        {
          "account_id": "rmlsnk.testnet",
          "amount": "'$FIVE_TOKENS'"
        },
        {
          "account_id": "participant_2.testnet",
          "amount": "'$ONE_TOKEN'"
        },
        {
          "account_id": "participant_4.testnet",
          "amount": "'$ONE_TOKEN'"
        }
    ],
    "token_id": "'$LNC'"
}' --accountId $USER_ACCOUNT2 --depositYocto 1 --gas $GAS

```
