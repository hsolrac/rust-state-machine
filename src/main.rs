mod balances; 
use balances::Pallet;

fn main() {
    let mut pallet = Pallet::new(); 
    pallet.set_balance("Account1".to_string(), 100);
    pallet.set_balance("Account2".to_string(), 100);

    pallet.transfer("Account1".to_string(), "Account2".to_string(), 500).unwrap();
    
    println!("Transfer done!");
}
