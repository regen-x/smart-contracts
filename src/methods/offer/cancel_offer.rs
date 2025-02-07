use soroban_sdk::{token, Env};
use crate::storage::{offer::{read_offer, cancel_offer as cancel_offer_storage}, types::error::Error};

pub(crate) fn cancel_offer(env: &Env, offer_id: &i128) -> Result<i128, Error> {
    let offer = read_offer(env, offer_id);

    let owner = offer.owner;
    owner.require_auth();

    let token_client = token::Client::new(env, &offer.token_address);
    token_client.transfer(&env.current_contract_address(), &owner, &offer.amount);
    
    cancel_offer_storage(env, offer_id);

    Ok(offer_id.clone())
}