use soroban_sdk::{token, Env};
use crate::storage::{offer::{cancel_offer as cancel_offer_storage, read_offer}, types::{error::Error, offer::Offer}};

pub(crate) fn cancel_offer(env: &Env, offer_id: &i128) -> Result<(i128, Offer), Error> {
    let (_, offer) = read_offer(env, offer_id);

    if !offer.is_active {
        return Err(Error::OfferNotActive);
    }

    let owner = offer.owner;
    owner.require_auth();

    let token_client = token::Client::new(env, &offer.token_address);
    token_client.transfer(&env.current_contract_address(), &owner, &offer.amount);
    
    cancel_offer_storage(env, offer_id);

    let cancelled_offer = read_offer(env, offer_id);

    Ok(cancelled_offer)
}