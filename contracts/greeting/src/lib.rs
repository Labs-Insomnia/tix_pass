#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, String,Vec,};

// const TITLE: Symbol = symbol_short!("TITLE");
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventOpts{
    name: String,
    amount_of_tix : u32,
    store : Vec<Address>
}

#[contract]
pub struct BaseContract;

#[contractimpl]
impl BaseContract {

    // pub fn set_title(env: Env, title: String) {
    //             env.storage().instance().set(&TITLE, &title)
    // }

    // pub fn read_title(env: Env) -> String {
    //     env.storage().instance().get(&TITLE)
    //         .unwrap_or(String::from_str(&env, "Default Title"))
    // }

    pub fn create_event_contract(env :Env, 
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<Val>, ){

    }
    
} 

mod test;
