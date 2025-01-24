use soroban_sdk::Env;
use crate::storage::admin::has_admin as has_admin_on_storage;

pub fn has_admin(env: &Env) -> bool {
    has_admin_on_storage(&env)
}
