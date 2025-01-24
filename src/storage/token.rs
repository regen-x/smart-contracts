use soroban_sdk::{Address, Env};

use super::types::{storage::DataKey, token::Token};

pub(crate) fn set_token(env: &Env, token: &Token) {
    let key = DataKey::Token(token.address.clone());

    env.storage().instance().set(&key, token);
}

pub(crate) fn read_token(env: &Env, address: &Address) -> Token {
    let key = DataKey::Token(address.clone());

    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn has_token(env: &Env, address: &Address) -> bool {
    let key = DataKey::Token(address.clone());

    env.storage().instance().has(&key)
}
