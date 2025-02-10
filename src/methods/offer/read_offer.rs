use soroban_sdk::Env;

use crate::storage::{offer::read_offer as read_offer_by_id, types::{error::Error, offer::Offer}};

pub(crate) fn read_offer(env: &Env, offer_id: &i128) -> Result<(i128, Offer), Error> {
     Ok(read_offer_by_id(env, offer_id))
}