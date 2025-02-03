use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct Token {
    pub address: Address,
    pub price: i128,
    pub supply: i128,
    pub owner: Address,
}
