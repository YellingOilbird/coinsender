#### Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
    `cargo build --target wasm32-unknown-unknown --release`

#### Contract deploy

```near dev-deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm```
```near deploy -f --wasmFile target/wasm32-unknown-unknown/release/coinsender.wasm --accountId v3coinsender.testnet```

-----------------------------------------------------------------------------------------------------------------
#### envs

```shell
#E24 decimals NEP-141 tokens
export CONTRACT_ID=v3coinsender.testnet
export AURORA=aurora.fakes.testnet
export WETH=weth.fakes.testnet
export PARAS=paras.fakes.testnet
export WNEAR=wrap.testnet
export DAI=dai.fakes.testnet
export USDC=usdc.fakes.testnet
export USN=usn.fakes.testnet
export CHEDDAR=token-v3.cheddar.testnet
export USDT=usdt.fakes.testnet
export P8=p8_token.testnet
export LNC=lnc.factory.tokenhub.testnet
export USER_ACCOUNT=rmlsnk.testnet
export USER_ACCOUNT2=participant_1.testnet
export GAS=300000000000000
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
near call $CONTRACT_ID set_owner '{"new_owner":"'$USER_ACCOUNT'"}' --accountId $CONTRACT_ID --gas $GAS --depositYocto 1
near call $CONTRACT_ID upgrade '' --accountId $USER_ACCOUNT --gas $GAS


#### tokens
```shell
near call $CONTRACT_ID whitelist_token '{"token_id": "'$USDC'"}' --accountId $CONTRACT_ID --depositYocto 1 --gas $GAS
near call $CONTRACT_ID remove_token '{"token_id": "'$CHEDDAR'"}' --accountId $CONTRACT_ID  --gas $GAS
near call $CONTRACT_ID get_whitelisted_tokens '' --accountId $CONTRACT_ID
```
#### vault
```shell
near call $P8 ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$HUNDRED_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
near call $REF ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$HUNDRED_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
near call $CONTRACT_ID deposit_near '' --accountId $USER_ACCOUNT --amount 100 
#view
near call $CONTRACT_ID get_deposit_by_token '{"account_id": "'$USER_ACCOUNT'", "token_id":"'$REF'"}' --accountId $CONTRACT_ID
near call $CONTRACT_ID get_user_vault '{"account_id": "'$USER_ACCOUNT'"}' --accountId $CONTRACT_ID
```
#### MULTISEND !!!
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
    "token_id": "'$P8'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
```


\\\ DEV \\\
```rust
near call $CONTRACT_ID get_user_vault '{"account_id": "'$USER_ACCOUNT'"}' --accountId $CONTRACT_ID
near call $CONTRACT_ID view_transfer_statuses '' --accountId $CONTRACT_ID

// deposit

near call $P8 ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$TEN_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS

//just send - 11 Tgas
near call $CONTRACT_ID send '{
  "receiver_id":"'$USER_ACCOUNT2'", 
  "amount":"'$TEN_TOKENS'", 
  "token_id":"'$P8'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
near call $CONTRACT_ID view_transfer_statuses '' --accountId $CONTRACT_ID

//checked send - 19 Tgas
near call $CONTRACT_ID checked_send '{
  "receiver_id":"'$LNC'", 
  "amount":"'$ONE_TOKEN'", 
  "token_id":"'$P8'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
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
