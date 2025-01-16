use soroban_sdk::{testutils::Address as _, token, Address, Env};

use crate::{contract::ContractClient, Contract};

use super::{constants::BASE_MINT_AMOUNT, utils::create_token_contract};

pub struct ContractTest<'a> {
    pub env: Env,
    pub contract: ContractClient<'a>,
    pub admin: Address,
    pub user_a: Address,
    pub user_b: Address,
    pub token: (token::Client<'a>, token::StellarAssetClient<'a>, Address),
}

impl<'a> ContractTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, Contract {});
        let contract = ContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token_issuer = Address::generate(&env);

        let user_a = Address::generate(&env);
        let user_b = Address::generate(&env);

        let (token_client, token_admin) = create_token_contract(&env, &token_issuer);

        token_admin.mint(&user_a, &BASE_MINT_AMOUNT);
        token_admin.mint(&user_b, &BASE_MINT_AMOUNT);

        ContractTest {
            env,
            contract,
            admin,
            user_a,
            user_b,
            token: (token_client, token_admin, token_issuer),
        }
    }
}
