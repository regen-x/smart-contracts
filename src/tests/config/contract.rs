use soroban_sdk::{testutils::Address as _, token, Address, Env};

use crate::{contract::ContractClient, Contract};

use super::{constants::BASE_MINT_AMOUNT, utils::create_token_contract};

pub struct ContractTest<'a> {
    pub env: Env,
    pub contract: ContractClient<'a>,
    pub admin: Address,
    pub user_a: Address,
    pub user_b: Address,
    pub user_c: Address,
    pub user_d: Address,
    pub reference_token: (token::Client<'a>, token::StellarAssetClient<'a>, Address),
    pub token: (token::Client<'a>, token::StellarAssetClient<'a>, Address),
}

impl<'a> ContractTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin: Address = Address::generate(&env);
        let reference_token_issuer = Address::generate(&env);
        let token_issuer = Address::generate(&env);
        let user_a = Address::generate(&env);
        let user_b = Address::generate(&env);
        let user_c = Address::generate(&env);
        let user_d = Address::generate(&env);

        let (token_client, token_admin) = create_token_contract(&env, &token_issuer);
        let (reference_token_client, reference_token_admin) = create_token_contract(&env, &reference_token_issuer);

        let contract_id = env.register(Contract, (&admin, &reference_token_client.address));
        let contract = ContractClient::new(&env, &contract_id);

        reference_token_admin.mint(&user_a, &BASE_MINT_AMOUNT);
        token_admin.mint(&admin, &BASE_MINT_AMOUNT);
        token_admin.mint(&user_c, &BASE_MINT_AMOUNT);
        reference_token_admin.mint(&user_c, &BASE_MINT_AMOUNT);
        reference_token_admin.mint(&user_d, &BASE_MINT_AMOUNT);

        ContractTest {
            env,
            contract,
            admin,
            user_a,
            user_b,
            user_c,
            user_d,
            reference_token: (reference_token_client, reference_token_admin, reference_token_issuer),
            token: (token_client, token_admin, token_issuer),
        }
    }
}
