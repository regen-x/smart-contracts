use soroban_sdk::{Address, Env};

use crate::storage::admin::get_admin as get_admin_on_storage;

pub fn get_admin(env: &Env) -> Address {
    get_admin_on_storage(&env)
}
