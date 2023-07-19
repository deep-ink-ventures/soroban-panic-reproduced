#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, panic_with_error, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Whatever = 2,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn raise_error(env: Env) {
        panic_with_error!(&env, Error::Whatever);
    }
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_panic() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);
    client.raise_error();
}