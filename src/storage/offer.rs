use soroban_sdk::Env;

use super::types::{offer::Offer, storage::DataKey};


pub(crate) fn set_offer(env: &Env, offer: &Offer) -> i128 {
    let offer_count_key = DataKey::OfferCount;
    let offer_count: i128 = env.storage().instance().get(&offer_count_key).unwrap();

    let offer_id = offer_count + 1;
    let key = DataKey::Offer(offer_id);

    env.storage().instance().set(&key, offer);
    set_offer_count(env, &offer_id);

    offer_id
}


pub(crate) fn read_offer(env: &Env, offer_id: &i128) -> Offer {
    let key = DataKey::Offer(offer_id.clone());

    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn set_offer_count(env: &Env, offer_count: &i128) {
    let key = DataKey::OfferCount;

    env.storage().instance().set(&key, offer_count);
}

pub(crate) fn update_offer(env: &Env, offer_id: &i128,  price: i128) -> i128 {
    let mut offer = read_offer(env, offer_id);
    let key = DataKey::Offer(offer_id.clone());

    offer.total_price = price;

    env.storage().instance().set(&key, &offer);
    
    offer_id.clone()
}

pub(crate) fn cancel_offer(env: &Env, offer_id: &i128) -> i128 {
    let mut offer = read_offer(env, offer_id);
    let key = DataKey::Offer(offer_id.clone());

    offer.is_active = false;
    env.storage().instance().set(&key, &offer);

    offer_id.clone()
}

pub(crate) fn buy_offer(env: &Env, offer_id: &i128) -> i128 {
    let mut offer = read_offer(env, offer_id);
    let key = DataKey::Offer(offer_id.clone());

    offer.amount = 0;
    offer.is_active = false;

    env.storage().instance().set(&key, &offer);

    offer_id.clone()
}

