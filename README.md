# COINSENDER
## Realised features:
#### contract side
- ##### *Rust smart-contract included:*
- Send functions which takes many accounts to transfer NEAR
- Send functions which takes many accounts to transfer FT
- Deposit with token options for more user-friendly interactions
- Whitelist token functionality with included registration in token storage for contract
- Multi-storage registration for many users per one transaction
- ```Vault``` which implements simple stats and update actions for checking occupation and reward users by activity in the NEAR future
- Contract works without initialization but owner can set after by call ```set_owner()``` from ```contract_id```

- ##### *WEB4*
- web4 implemented
- Front-end side creates GET links which retrieved by contract web4 module and processing to show many views
- User balance comes directly from Rust

- ##### *JS(Front-end)
- Custom function for convertation choosed token to yocto amounts based on ```formatNearAmount``` from ```near-api-js```
- 90-s styled design from https://nostalgic-css.github.io/NES.css/[NES-CSS]
- Sequence of state changes using only ```localStorage```
- When token whitelisted by owner he actually comes to index page as pretty-view with icon and also as SELECT option


#### Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
    `cargo build --target wasm32-unknown-unknown --release`

#### Contract deploy

```near dev-deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm```
<<<<<<< Updated upstream
```near deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm --accountId v3coinsender.testnet```
=======
```near deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm --accountId $CONTRACT_ID```

>>>>>>> Stashed changes

-----------------------------------------------------------------------------------------------------------------
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
near call $CONTRACT_ID whitelist_token '{"token_id": "'$REF'"}' --accountId $CONTRACT_ID --depositYocto 1 --gas $GAS
near call $CONTRACT_ID remove_token '{"token_id": "'$REF'"}' --accountId $CONTRACT_ID  --gas $GAS
near call $CONTRACT_ID get_whitelisted_tokens '' --accountId $CONTRACT_ID
```
#### vault
```shell
near call $REF ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$TEN_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
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
          "account_id": "learnclub.testnet",
          "amount": "'$ONE_TOKEN'"
        },
        {
          "account_id": "blaze.testnet",
          "amount": "'$ONE_TOKEN'"
        },
        {
          "account_id": "coldy.testnet",
          "amount": "'$ONE_TOKEN'"
        }
    ],
    "token_id": "'$REF'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
<<<<<<< Updated upstream
near call $CONTRACT_ID view_transfer_statuses '' --accountId $CONTRACT_ID
// unsafe - 13 Tgas / for many - 2+14 + 4n ( for 4 ~ 30Tgas)
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
    "token_id": "'$P8'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
// callbacked send - 19 Tgas / for many - 2+24 + 5n (for 4 ~50Tgas)
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

near call p8_token.testnet storage_balance_of '{"account_id":"rncy.testnet"}' --accountId rmlsnk.testnet
=======
```
>>>>>>> Stashed changes
