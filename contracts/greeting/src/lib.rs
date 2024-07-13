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
            // Skip authorization if deployer is the current contract.
        if deployer != env.current_contract_address() {
            deployer.require_auth();
        }
        // Deploy the contract using the uploaded Wasm with given hash.
        let deployed_address = env
            .deployer()
            .with_address(deployer, salt)
            .deploy(wasm_hash);

         // Invoke the init function with the given arguments.
         let res: Val = env.invoke_contract(&deployed_address, &init_fn, init_args);

    }
    
} 

mod test;
