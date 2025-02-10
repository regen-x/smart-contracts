use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::{
    methods::{
        admin::{get_admin::get_admin, has_admin::has_admin, set_admin::set_admin}, offer::{buy_offer::buy_offer, cancel_offer::cancel_offer, create_offer::create_offer, read_offer::read_offer, update_offer_price::update_offer_price}, public::{initialize::initialize, transfer::transfer}, token::{issue_token::issue_token, set_reference_token::set_reference_token}
    },

    storage::types::{error::Error, offer::Offer, reference_token::ReferenceToken, token::Token},

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

    pub fn create_offer(
        env: Env,
        token_address: Address,
        amount: i128,
        price: i128,
        owner: Address,
    ) -> Result<(i128, Offer), Error> {
        create_offer(&env, &Offer{
            token_address, amount, total_price: price, owner, is_active: true
        })
    }

    pub fn update_offer(
        env: Env,
        offer_id: i128,
        price: i128,
    ) -> Result<(i128, Offer), Error> {
        update_offer_price(&env, &offer_id, price)
    }

    pub fn cancel_offer(
        env: Env,
        offer_id: i128,
    ) -> Result<(i128, Offer), Error> {
        cancel_offer(&env, &offer_id)
    }

    pub fn buy_offer(
        env: Env,
        offer_id: i128,
        buyer: Address,
    ) -> Result<(i128, Offer), Error> {
        buy_offer(&env, &offer_id, &buyer)
    }

    pub fn has_admin(env: Env) -> bool {
        has_admin(&env)
    }

    pub fn read_offer(env: Env, offer_id: i128) -> Result<(i128, Offer), Error> {
        read_offer(&env, &offer_id)
    }


    pub fn get_admin(env: Env) -> Address {
        get_admin(&env)
    }
}
