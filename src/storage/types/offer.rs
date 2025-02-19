use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]

pub struct Offer {
    pub token_address: Address,
    pub total_price: i128,
    pub amount: i128,
    pub owner: Address,
    pub is_active: bool,
}
