use soroban_sdk::{token, Env};
use crate::storage::{offer::{cancel_offer as cancel_offer_storage, read_offer}, types::{error::Error, offer::Offer}};

pub(crate) fn cancel_offer(env: &Env, offer_id: &i128) -> Result<(i128, Offer), Error> {
    let offer = read_offer(env, offer_id);

    let owner = offer.1.owner;
    owner.require_auth();

    let token_client = token::Client::new(env, &offer.1.token_address);
    token_client.transfer(&env.current_contract_address(), &owner, &offer.1.amount);
    
    cancel_offer_storage(env, offer_id);

    let cancelled_offer = read_offer(env, offer_id);

    Ok(cancelled_offer)
}