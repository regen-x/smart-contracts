use soroban_sdk::Env;
use crate::storage::{offer::{read_offer, update_offer}, types::{error::Error, offer::Offer}};

pub(crate) fn update_offer_price(env: &Env, offer_id: &i128, price: i128) -> Result<(i128, Offer), Error> {
    let (_, offer) = read_offer(env, offer_id);

    let owner = offer.owner;
    owner.require_auth();
   
    update_offer(env, offer_id, price);

    let updated_offer = read_offer(env, offer_id);

    Ok(updated_offer)
}
