use crate::tests::config::constants::AMOUNT_MULTIPLIER;
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

    let token_price = 2 * AMOUNT_MULTIPLIER;
    let token_supply = BASE_MINT_AMOUNT;

    assert_eq!(token_client.balance(&admin), token_supply);
    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);

    let transfer_amount = 2 * AMOUNT_MULTIPLIER;

    assert_eq!(reference_token_client.balance(&user_a), BASE_MINT_AMOUNT);
    assert_eq!(token_client.balance(&contract.address), token_supply);

    contract.transfer(&user_a, &token_client.address, &transfer_amount);

    assert_eq!(token_client.balance(&user_a), transfer_amount);
    assert_eq!(
        reference_token_client.balance(&user_a),
        BASE_MINT_AMOUNT - ((transfer_amount/ AMOUNT_MULTIPLIER) * token_price)
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

#[test]
fn create_offer_test() {
    let ContractTest {
        contract,
        user_c,
        user_b,
        token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();
    let (token_client, _, __) = token;

    let token_price = 10;
    let token_supply = 1000;

    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);


    let total_balance = token_client.balance(&user_c);

    let token_amount_to_sell = 5;
    let token_price_to_sell = 10;

    let offer_id = contract.create_offer(&token_client.address, &token_amount_to_sell, &token_price_to_sell, &user_c);

    assert_eq!(token_client.balance(&user_c), total_balance - token_amount_to_sell);

    let offer = contract.read_offer(&offer_id);

    assert_eq!(offer.amount, token_amount_to_sell);
    assert_eq!(offer.total_price, token_price_to_sell);
    assert_eq!(offer.owner, user_c);
    assert_eq!(offer.token_address, token_client.address);
    assert_eq!(offer.is_active, true);
}

#[test]
fn buy_token_test() {
    let ContractTest {
        contract,
        user_c,
        user_b,
        user_d,
        token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    let (token_client, _, __) = token;

    let token_price = 10;
    let token_supply = 1000;

    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);

    let token_amount_to_sell = 5;
    let token_price_to_sell = 10;
    let total_balance = token_client.balance(&user_c);

    let offer_id = contract.create_offer(&token_client.address, &token_amount_to_sell, &token_price_to_sell, &user_c);

    contract.buy_offer(&offer_id,  &user_d);
    
    assert_eq!(token_client.balance(&user_d), token_amount_to_sell);
    assert_eq!(token_client.balance(&user_c), total_balance - token_amount_to_sell);
}

#[test]
fn cancel_offer_test() {
    let ContractTest {
        contract,
        user_c,
        user_b,
        token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    let (token_client, _, __) = token;

    let token_price = 10;
    let token_supply = 1000;

    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);
    
    let token_amount_to_sell = 5;
    let token_price_to_sell = 10;

    let total_balance = token_client.balance(&user_c);

    let offer_id = contract.create_offer(&token_client.address, &token_amount_to_sell, &token_price_to_sell, &user_c);

    contract.cancel_offer(&offer_id);

    let offer = contract.read_offer(&offer_id);

    assert_eq!(token_client.balance(&user_c), total_balance);

    assert_eq!(offer.is_active, false);
}

#[test]
fn update_offer_test() {
    let ContractTest {
        contract,
        user_c,
        user_b,
        token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    let (token_client, _, __) = token;

    let token_price = 10;
    let token_supply = 1000;

    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);  
    
    let token_amount_to_sell = 5;
    let token_price_to_sell = 10;

    let offer_id = contract.create_offer(&token_client.address, &token_amount_to_sell, &token_price_to_sell, &user_c);

    let new_token_price_to_sell = 20;

    contract.update_offer(&offer_id,  &new_token_price_to_sell);

    let offer = contract.read_offer(&offer_id);

    assert_eq!(offer.total_price, new_token_price_to_sell);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn create_offer_with_amount_zero() {
    let ContractTest {
        contract,
        user_c,
        user_b,
        token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    env.mock_all_auths();
    let (token_client, _, __) = token;

    let token_price = 10;
    let token_supply = 1000;

    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);

    let token_amount_to_sell = 0;
    let token_price_to_sell = 10;

    contract.create_offer(&token_client.address, &token_amount_to_sell, &token_price_to_sell, &user_c);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn buy_inactive_offer() {
    let ContractTest {
        contract,
        user_c,
        user_b,
        user_d,
        token,
        env,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    let (token_client, _, __) = token;

    let token_price = 10;
    let token_supply = 1000;

    contract.issue_token(&token_client.address, &token_price, &token_supply, &user_b);

    let token_amount_to_sell = 5;
    let token_price_to_sell = 10;

    let offer_id = contract.create_offer(&token_client.address, &token_amount_to_sell, &token_price_to_sell, &user_c);

    contract.cancel_offer(&offer_id);

    contract.buy_offer(&offer_id, &user_d);
}   


#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn create_offer_for_non_existing_token() {
    let ContractTest {
        contract,
        user_c,
        env,
        token,
        ..
    } = ContractTest::setup();
    env.mock_all_auths();

    let (token_client, _, __) = token;

    let token_amount_to_sell = 5;
    let token_price_to_sell = 10;

    contract.create_offer(&token_client.address, &token_amount_to_sell, &token_price_to_sell, &user_c);
}

