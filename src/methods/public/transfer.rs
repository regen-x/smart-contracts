use soroban_sdk::{token, Address, Env};
use crate::storage::{reference_token::get_reference_token, token::read_token, types::error::Error, admin::get_admin};

pub fn transfer(
    env: &Env,
    owner: Address,
    investor: Address,
    token_address: Address,
    amount: i128,
) -> Result<(), Error> {
    let admin = get_admin(env);

    admin.require_auth();
    investor.require_auth();

    let token = read_token(env, &token_address);
    let reference_token = get_reference_token(env);
    let reference_amount = amount * token.price;

    let reference_client = token::Client::new(env, &reference_token.address);

    if reference_client.balance(&investor) < reference_amount {
        return Err(Error::InsufficientBalance);
    }

    reference_client.transfer(&investor, &owner, &reference_amount);

    let token_client = token::Client::new(env, &token_address);

    if token_client.balance(&admin) < amount {
        return Err(Error::InsufficientBalance);
    }

    token_client.transfer(&admin, &investor, &amount);

    Ok(())
}