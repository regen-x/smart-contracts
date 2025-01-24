use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NonExistentUser = 0,
    TokenNotFound = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
    TokenAlreadyExists = 4,
}
