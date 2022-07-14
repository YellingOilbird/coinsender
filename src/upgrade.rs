use crate::*;

#[near_bindgen]
impl Contract {
    /// Using for call upgrade
    #[allow(unused)]
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.get_owner(),
            "{}", ERR_NOT_ALLOWED
        );
    }
    /// Using for call internal methods
    pub(crate) fn assert_owner_or_self(&self) -> bool {
        env::predecessor_account_id() == env::current_account_id()
            ||  env::predecessor_account_id() == self.get_owner()
    }
    /// Set owner for contract - by default is None
    #[payable]
    pub fn set_owner(&mut self, new_owner: AccountId) {
        assert_one_yocto();
        assert!(self.assert_owner_or_self(), "{}", ERR_NOT_ALLOWED);

        self.owner_id = Some(new_owner);
        log!("@{} Setting contract owner: @{} ", env::predecessor_account_id(), self.get_owner());
    }
    pub fn get_owner(&self) -> AccountId {
        if let Some(owner) = self.owner_id.clone() {
            owner
        } else {
            panic!("{}", ERR_OWNER_NOT_SET)
        }
    }
    #[payable]
    /// Method to fill Account 
    pub fn transfer_near_to_contract(&mut self) {
        let attached_deposit = env::attached_deposit();
        assert!(attached_deposit > 0, "ERR_NEGATIVE_DEPOSIT");
        Promise::new(env::signer_account_id())
                    .transfer(attached_deposit);
        log!(
            "@{} transfer {} yoctoNEAR to contract balance", 
            env::signer_account_id(),
            attached_deposit
        )
    }
    /// A method to migrate a state during the contract upgrade.
    /// Can only be called after upgrade method.
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let contract: Self = env::state_read().expect(ERR_STATE_NOT_INITIALIZED);
        contract
    }

    /// Returns semver of this contract.
    pub fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

#[cfg(target_arch = "wasm32")]
mod upgrade {
    use super::*;
    use near_sdk::env;
    use near_sdk::Gas;
    use near_sys as sys;
    /// Self upgrade and call migrate, optimizes gas by not loading into memory the code.
    /// Takes as input non serialized set of bytes of the code.
    /// After upgrade we call *pub fn migrate()* on the NEW CONTRACT CODE
    #[no_mangle]
    pub fn upgrade() {
        /// Rest after deploy call gas for migration call. One Tera - 1 TGas
        const GAS_FOR_UPGRADE: Gas = Gas(Gas::ONE_TERA.0 * 20);
        env::setup_panic_hook();

        assert!(env::prepaid_gas() >= Gas(150 * Gas::ONE_TERA.0),"{}", ERR_EXCEEDED_OF_PREPAID_GAS);

        // Assert ownership
        let contract: Contract = env::state_read().expect(ERR_STATE_NOT_INITIALIZED);
        contract.assert_owner();

        let current_id = env::current_account_id();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE;
        unsafe {
            // Load input (NEW CONTRACT CODE) into register 0.
            sys::input(0);
            // prepare self-call promise
            let promise_id = sys::promise_batch_create(current_id.as_bytes().len() as _, current_id.as_bytes().as_ptr() as _);
            
            // #Action_1 - deploy/upgrade code from register 0
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
            // #Action_2 - schedule a call for migrate
            // Execute on NEW CONTRACT CODE
            sys::promise_batch_action_function_call(
                    promise_id,
                    migrate_method_name.len() as _,
                    migrate_method_name.as_ptr() as _,
                    0 as _,
                    0 as _,
                    0 as _,
                    u64::from(attached_gas),
                );
        }
    }
}