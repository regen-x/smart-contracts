use crate::tests::config::contract::ContractTest;
use crate::{contract::ContractClient, tests::config::constants::BASE_MINT_AMOUNT, Contract};
use soroban_sdk::testutils::Events;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn initialize_test() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reference_token_issuer = Address::generate(&env);

    let initial_events: soroban_sdk::Vec<(
        Address,
        soroban_sdk::Vec<soroban_sdk::Val>,
        soroban_sdk::Val,
    )> = env.events().all().clone();
    assert_eq!(initial_events.len(), 0);

    let contract_id = env.register(Contract, (&admin, &reference_token_issuer));
    let contract = ContractClient::new(&env, &contract_id);

    assert!(contract.has_admin());
    assert_eq!(contract.get_admin(), admin);
}

#[test]
#[should_panic(expected = "Error(Value, InvalidInput)")]
fn initialize_fail_test() {
    let env = Env::default();
    env.mock_all_auths();

    let invalid_admin = "TEST";
    let reference_token_issuer = Address::generate(&env);

    env.register(
        Contract,
        (
            Address::from_str(&env, &invalid_admin),
            &reference_token_issuer,
        ),
    );
}

#[test]
fn set_admin_test() {
    let ContractTest {
        contract, user_a, ..
    } = ContractTest::setup();

    contract.set_admin(&user_a);
}

#[test]
fn set_reference_token_test() {
    let ContractTest {
        contract, user_a, ..
    } = ContractTest::setup();

    contract.set_reference_token(&user_a);
}

#[test]
fn transfer_test() {
    let ContractTest {
        contract,
        user_a,
        user_b,
        admin,
        token,
        reference_token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    let (token_client, _, __) = token;
    let (reference_token_client, _, _) = reference_token;

    let token_price = 2;
    let token_supply = BASE_MINT_AMOUNT;

    assert_eq!(token_client.balance(&admin), token_supply);
    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);

    let transfer_amount = 2;

    assert_eq!(reference_token_client.balance(&user_a), BASE_MINT_AMOUNT);
    assert_eq!(token_client.balance(&contract.address), token_supply);

    contract.transfer(&user_a, &token_client.address, &transfer_amount);

    assert_eq!(token_client.balance(&user_a), transfer_amount);
    assert_eq!(
        reference_token_client.balance(&user_a),
        BASE_MINT_AMOUNT - (transfer_amount * token_price)
    );
    assert_eq!(
        token_client.balance(&contract.address),
        token_supply - transfer_amount
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn transfer_insufficient_balance_investor() {
    let ContractTest {
        contract,
        user_a,
        user_b,
        token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    let (token_client, _, __) = token;

    let token_price = 2;
    let token_supply = 1000;

    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);

    let transfer_amount = BASE_MINT_AMOUNT + 1;

    contract.transfer(&user_a, &token_client.address, &transfer_amount);
}
