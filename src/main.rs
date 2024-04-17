mod balances; 
use balances::Pallet;

fn main() {
    let mut pallet = Pallet::new(); 
    pallet.set_balance("Carlos".to_string(), 10);

    let balance = pallet.get_balance("Carlos".to_string());
    println!("Balance: {}", balance);
    println!("Hello, world!");
}
