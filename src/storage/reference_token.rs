use soroban_sdk::Env;

use super::types::{reference_token::ReferenceToken, storage::DataKey};

pub(crate) fn set_reference_token(env: &Env, reference_token: &ReferenceToken) {
    let key = DataKey::ReferenceToken;

    env.storage().instance().set(&key, reference_token);
}

pub(crate) fn get_reference_token(env: &Env) -> ReferenceToken {
    let key = DataKey::ReferenceToken;

    env.storage().instance().get(&key).unwrap()
}