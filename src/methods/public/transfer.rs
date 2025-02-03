use soroban_sdk::{token, Address, Env};
use crate::storage::{reference_token::get_reference_token, token::read_token, types::error::Error};

pub fn transfer(
    env: &Env,
    investor: Address,
    token_address: Address,
    amount: i128,
) -> Result<(), Error> {
    investor.require_auth();

    let token = read_token(env, &token_address);
    let reference_token = get_reference_token(env);
    let reference_amount = amount * token.price;

    let reference_client = token::Client::new(env, &reference_token.address);

    if reference_client.balance(&investor) < reference_amount {
        return Err(Error::InsufficientBalance);
    }
    
    let token_client = token::Client::new(env, &token_address);
    
    if token_client.balance(&env.current_contract_address()) < amount {
        return Err(Error::InsufficientBalance);
    }
    
    reference_client.transfer(&investor, &token.owner, &reference_amount);
    
    token_client.transfer(&env.current_contract_address(), &investor, &amount);

    Ok(())
}