use crate::storage::types::error::Error;
use crate::storage::{
    admin::get_admin,
    reference_token::set_reference_token as set_reference_token_on_storage,
    types::reference_token::ReferenceToken,
};
use soroban_sdk::{Address, Env};

pub(crate) fn set_reference_token(
    env: &Env,
    reference_token: &ReferenceToken,
) -> Result<ReferenceToken, Error> {
    let admin: Address = get_admin(env);
    admin.require_auth();

    set_reference_token_on_storage(env, reference_token);
    Ok(reference_token.clone())
}
