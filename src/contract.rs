use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::{
    methods::{
        admin::{get_admin::get_admin, has_admin::has_admin, set_admin::set_admin},
        public::{initialize::initialize, transfer::transfer},
        token::{issue_token::issue_token, set_reference_token::set_reference_token},
    },
    storage::types::{error::Error, reference_token::ReferenceToken, token::Token},
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, admin: Address, reference_token: Address) -> Result<(), Error> {
        initialize(&env, admin, reference_token)
    }

    pub fn set_admin(env: Env, admin: Address) -> Result<Address, Error> {
        set_admin(&env, admin)
    }

    pub fn set_reference_token(env: Env, token_address: Address) -> Result<ReferenceToken, Error> {
        set_reference_token(
            &env,
            &ReferenceToken {
                address: token_address,
            },
        )
    }

    pub fn issue_token(
        env: Env,
        token: Address,
        price: i128,
        supply: i128,
        owner: Address,
    ) -> Result<Token, Error> {
        issue_token(
            &env,
            &Token {
                address: token,
                price,
                supply,
                owner,
            },
        )
    }

    pub fn transfer(
        env: Env,
        investor: Address,
        token_address: Address,
        amount: i128,
    ) -> Result<(), Error> {
        transfer(&env, investor, token_address, amount)
    }

    pub fn has_admin(env: Env) -> bool {
        has_admin(&env)
    }

    pub fn get_admin(env: Env) -> Address {
        get_admin(&env)
    }
}
