use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::set_admin,
        reference_token::set_reference_token,
        types::{error::Error, reference_token::ReferenceToken},
    },
};

pub fn initialize(env: &Env, admin: Address, reference_token: Address) -> Result<(), Error> {
    admin.require_auth();
    
    set_admin(env, &admin);
    set_reference_token(env, &ReferenceToken { address: reference_token });

    events::contract::contract_initialized(env, &admin);
    Ok(())
}
