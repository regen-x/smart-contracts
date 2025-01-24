use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct ReferenceToken {
    pub address: Address,
}
