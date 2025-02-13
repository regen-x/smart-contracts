use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token(Address),
    ReferenceToken,
    Offer(i128),
    OfferCount
}
