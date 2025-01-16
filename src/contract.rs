use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::{
    methods::{
        admin::set_admin::set_admin,
        public::{initialize::initialize, transfer::transfer},
        user::user::{get_user, update_transfer_amount},
    },
    storage::types::{error::Error, user::User},
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        initialize(&env, admin)
    }

    pub fn set_admin(env: Env, admin: Address) -> Result<Address, Error> {
        set_admin(&env, admin)
    }

    pub fn transfer(env: Env, from: Address, to: Address, token: Address, amount: i128) -> i128 {
        update_transfer_amount(&env, &from);

        transfer(&env, from, to, token, amount)
    }

    pub fn get_user(env: Env, address: Address) -> Result<User, Error> {
        get_user(&env, &address)
    }
}
