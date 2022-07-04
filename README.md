#### Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
    `cargo build --target wasm32-unknown-unknown --release`

#### Contract deploy
```near dev-deploy -f --wasmFile target/wasm32-unknown-unknown/release/near_basic_contract.wasm```
-----------------------------------------------------------------------------------------------------------------
#### envs

```shell
#E24 decimals NEP-141 tokens
export CONTRACT_ID=dev-1656601236767-19477293101338
export CHEDDAR=token-v3.cheddar.testnet
export REF=ref.fakes.testnet
export P8=p8_token.testnet
export LNC=lnc.factory.tokenhub.testnet
export USER_ACCOUNT=rmlsnk.testnet
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

#### tokens
```shell
near call $CONTRACT_ID whitelist_token '{"token_id": "'$CHEDDAR'"}' --accountId $CONTRACT_ID --depositYocto 1 --gas $GAS
near call $CONTRACT_ID get_whitelisted_tokens '' --accountId $CONTRACT_ID
```
#### vault
```shell
near call $LNC ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$TEN_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
near call $REF ft_transfer_call '{"receiver_id":"'$CONTRACT_ID'","amount": "'$HUNDRED_TOKENS'", "msg":"deposit"}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
near call $CONTRACT_ID deposit_near '' --accountId $USER_ACCOUNT --amount 100 
#view
near call $CONTRACT_ID get_deposit_by_token '{"account_id": "'$USER_ACCOUNT'", "token_id":"'$CHEDDAR'"}' --accountId $CONTRACT_ID
near call $CONTRACT_ID get_user_vault '{"account_id": "'$USER_ACCOUNT'"}' --accountId $CONTRACT_ID
```
#### MULTISEND !!!
```shell
near call $CONTRACT_ID multi_storage_deposit '{
    "accounts": ["participant_1.testnet","participant_2.testnet","participant_3.testnet","participant_4.testnet","lero4ka.testnet"],
    "token_id": "'$CHEDDAR'"
}' --accountId $USER_ACCOUNT --amount 1 --gas $GAS

near call $CONTRACT_ID multisend_from_balance '{
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
    "token_id": "'$CHEDDAR'"
}' --accountId $USER_ACCOUNT --depositYocto 1 --gas $GAS
```
