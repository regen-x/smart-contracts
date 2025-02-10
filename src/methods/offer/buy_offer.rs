use soroban_sdk::{token, Address, Env};

use crate::storage::{offer::{buy_offer as buy_offer_storage, read_offer}, reference_token::get_reference_token, types::{error::Error, offer::Offer}};



pub(crate) fn buy_offer(env: &Env, offer_id: &i128, buyer: &Address) -> Result<(i128, Offer), Error> {
    let (_, offer) = read_offer(env, offer_id);

    if !offer.is_active {
        return Err(Error::OfferNotActive);
    }

    let owner = offer.owner;

    buyer.require_auth();

    let reference_token = get_reference_token(env);

    let reference_client = token::Client::new(env, &reference_token.address);
    reference_client.transfer(&buyer, &owner, &offer.total_price);

    let token_client = token::Client::new(env, &offer.token_address);
    token_client.transfer(&env.current_contract_address(), &buyer, &offer.amount);

    let offer = buy_offer_storage(env, offer_id);

    Ok(offer)
}
