use soroban_sdk::testutils::Events;

use crate::tests::config::{constants::BASE_MINT_AMOUNT, contract::ContractTest};

#[test]
fn initialize_test() {
    let ContractTest {
        env,
        contract,
        admin,
        ..
    } = ContractTest::setup();

    let initial_events = env.events().all().clone();
    assert_eq!(initial_events.len(), 3);

    contract.initialize(&admin);

    assert_eq!(env.events().all().len(), initial_events.len() + 1);
}

#[test]
#[should_panic = "Error(Contract, #0)"]
fn initialize_fail_test() {
    let ContractTest {
        contract,
        admin,
        user_a,
        ..
    } = ContractTest::setup();

    contract.initialize(&admin);
    contract.initialize(&user_a);
}

#[test]
fn set_admin_test() {
    let ContractTest {
        contract,
        admin,
        user_a,
        ..
    } = ContractTest::setup();

    contract.initialize(&admin);
    contract.set_admin(&user_a);
}

#[test]
#[should_panic = "Error(Contract, #1)"]
fn set_admin_fail_test() {
    let ContractTest {
        contract, user_a, ..
    } = ContractTest::setup();

    contract.set_admin(&user_a);
}

#[test]
fn transfer_test() {
    let ContractTest {
        contract,
        user_a,
        user_b,
        token,
        ..
    } = ContractTest::setup();

    let (token_client, _, __) = token;

    let transfer_amount = BASE_MINT_AMOUNT / 2;

    assert_eq!(token_client.balance(&user_a), BASE_MINT_AMOUNT);
    assert_eq!(token_client.balance(&user_b), BASE_MINT_AMOUNT);

    contract.transfer(&user_a, &user_b, &token_client.address, &transfer_amount);

    assert_eq!(
        token_client.balance(&user_a),
        BASE_MINT_AMOUNT - transfer_amount
    );
    assert_eq!(
        token_client.balance(&user_b),
        BASE_MINT_AMOUNT + transfer_amount
    );
}

#[test]
fn get_user_test() {
    let ContractTest {
        contract,
        user_a,
        user_b,
        token,
        ..
    } = ContractTest::setup();

    let (token_client, _, __) = token;

    let transfer_amount = BASE_MINT_AMOUNT / 2;

    contract.transfer(&user_a, &user_b, &token_client.address, &transfer_amount);

    let user_a_historical_data = contract.get_user(&user_a);
    assert_eq!(user_a_historical_data.transactions_count, 1);
    contract.transfer(&user_a, &user_b, &token_client.address, &transfer_amount);

    let user_a_historical_data_2 = contract.get_user(&user_a);
    assert_eq!(user_a_historical_data_2.transactions_count, 2);
}

#[test]
#[should_panic = "Error(Contract, #2)"]
fn get_user_fail_test() {
    let ContractTest {
        contract, user_a, ..
    } = ContractTest::setup();

    contract.get_user(&user_a);
}
