mod balances; 
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<types::AccountId, types::Balance>,
    system: system::Pallet,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}


fn main() {
    let mut runtime = Runtime::new();
    let account_1 = "account_1".to_string();
    let account_2 = "account_2".to_string();
    let account_3 = "account_3".to_string();

    runtime.balances.set_balance(&account_1.clone(), 100);

    // start block
    runtime.system.increment_block_number(); 

    //transactions
    runtime.system.increment_nonce(account_1.clone());
    let _result = runtime.balances
        .transfer(account_1.clone(), account_2.clone(), 30)
        .map_err(|e| println!("transfer failed: {:?}", e));

    runtime.system.increment_nonce(account_2.clone());
    let _result = runtime.balances
        .transfer(account_1.clone(), account_3.clone(), 20)
        .map_err(|e| println!("transfer failed: {:?}", e));

    println!("{:#?}", runtime);
}
