# COINSENDER
#### see user vault web4 example on https://coinsender.testnet.page/processing/finality/rmlsnk.testnet where u can paste your user account to see stats
#### see TODO file for progress
### links:
https://coinsender.testnet.page - testnet  
https://coinsender.near.page - mainnet (in deployment)

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

- ### *JS(Front-end)
- Custom function for convertation choosed token to yocto amounts based on ```formatNearAmount``` from ```near-api-js```
- 90-s styled design from https://nostalgic-css.github.io/NES.css/[NES-CSS]
- Sequence of state changes using only ```localStorage```
- When token whitelisted by owner he actually comes to index page as pretty-view with icon and also as SELECT option


### Unimplemented (in-deploy)
This is my first experience with front-end side and it goes little hard. But JS part is 70% done (still with several bugs on it).
But I'm on the way to finish web and go to upgrade user vault functions and introduce rewards for most activity senders.  For example - top 10 senders by number of send and by summ in USN will be rewarded with free-month without fees / 100 sends without fees, etc.


#### Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
    `cargo build --target wasm32-unknown-unknown --release`

#### Contract deploy

```bash
near dev-deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm
near deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm --accountId coinsender.testnet
```
----------------------------------------------------------------------------------------------------------------
#### envs

```shell
export CONTRACT_ID=coinsender.testnet
export AURORA=aurora.fakes.testnet
export WETH=weth.fakes.testnet
export PARAS=paras.fakes.testnet
export WNEAR=wrap.testnet
export DAI=dai.fakes.testnet
export USDC=usdc.fakes.testnet
export USN=usn.fakes.testnet
export CHEDDAR=token-v3.cheddar.testnet
export USDT=usdt.fakes.testnet
export LNC=lnc.factory.tokenhub.testnet
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

#### tokens
```shell
near call $CONTRACT_ID whitelist_token '{"token_id": "'$LNC'"}' --accountId $CONTRACT_ID --depositYocto 1 --gas $GAS
near call $CONTRACT_ID remove_token '{"token_id": "'$REF'"}' --accountId $CONTRACT_ID  --gas $GAS
near call $CONTRACT_ID get_whitelisted_tokens '' --accountId $CONTRACT_ID
```
#### vault
```shell
near call $CONTRACT_ID deposit_near '' --accountId $USER_ACCOUNT --amount 2 --gas $GAS
near call $LNC ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$ONE_TOKEN'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
near call $REF ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$HUNDRED_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
//TODO FIX WITHDRAW TO PREDECCESSOR
near call $CONTRACT_ID withdraw_all '{
  "account_id":"'$USER_ACCOUNT'"
} ' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS

#view
near call $CONTRACT_ID get_deposit_by_token '{"account_id": "'$USER_ACCOUNT'", "token_id":"'$REF'"}' --accountId $CONTRACT_ID
near call $CONTRACT_ID get_user_vault '{"account_id": "'$USER_ACCOUNT'"}' --accountId $CONTRACT_ID
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
          "account_id": "participant_1.testnet",
          "amount": "'$ONE_TOKEN'"
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
    "token_id": "'$P8'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS

```
