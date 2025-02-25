#![cfg(test)]
use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}

#[test]
fn test_get_current_calue() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.get_current_value(), 0);
    
    assert_eq!(client.increment(), 1);
    assert_eq!(client.get_current_value(), 1);
    
    assert_eq!(client.increment(), 2);
    assert_eq!(client.get_current_value(), 2);
}


#[test]
#[should_panic(expected = "cant decrement when count is 0")]
fn test_decrement_panic() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    client.decrement();
}


#[test]
#[should_panic(expected = "cant decrement when count is 0")]
fn test_increment_decrement() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    assert_eq!(client.decrement(), 0);
    client.decrement();
}

#[test]
fn test_reset() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    client.reset();
    assert_eq!(client.get_current_value(), 0);
    
    assert_eq!(client.increment(), 1);
    assert_eq!(client.get_current_value(), 1);
}