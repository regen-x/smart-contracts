use crate::storage::types::error::Error;
use crate::storage::{admin::get_admin, token::{has_token,set_token as set_token_on_storage}};
use crate::storage::types::token::Token;
use soroban_sdk::{Address, Env};

pub(crate) fn write_token(env: &Env, token: &Token) -> Result<Token, Error> {
    if has_token(env, &token.address) {
        return Err(Error::TokenAlreadyExists);
    }

    let admin: Address = get_admin(env);
    admin.require_auth();

    set_token_on_storage(env, token);
    Ok(token.clone())
}
