use crate::storage::types::error::Error;
use crate::storage::{token::has_token, offer::set_offer};
use crate::storage::types::offer::Offer;
use soroban_sdk::{token, Env};

pub(crate) fn create_offer(env: &Env, offer: &Offer) -> Result<(i128, Offer), Error> {
    let owner = &offer.owner;
    owner.require_auth();

    if offer.amount == 0 {
        return Err(Error::OfferAmountZero);
    }

    if !has_token(env, &offer.token_address) {
        return Err(Error::TokenNotFound);
    }
    
    let token_client = token::Client::new(env, &offer.token_address);
    token_client.transfer(&owner, &env.current_contract_address(), &offer.amount);

    let created_offer = set_offer(env, offer);

    Ok(created_offer)
}
