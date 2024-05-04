use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

type AccountId = String; 
type Balance = u128;

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>where 
    AccountId: Ord + Clone,
    Balance: Zero + CheckedAdd + CheckedSub + Copy + std::cmp::PartialOrd,
{
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, account: &AccountId, amount: Balance) {
        self.balances.insert(&account, amount);
    }

    pub fn get_balance(&self, account: AccountId) -> Balance {
        *self.balances.get(&account).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance)-> Result<(), &'static str> {
        if self.get_balance(from.clone()) < amount {
            return Err("Inssuficient balance");
        }
        self.set_balance(&from.clone(), self.get_balance(from) - amount);
        self.set_balance(&to.clone(), self.get_balance(to) + amount);
        
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn set_balance_test() {
        let mut balances = super::Pallet::new();
        balances.set_balance(&"Account1".to_string(), 10);
        assert_eq!(balances.get_balance("Account1".to_string()), 10);
    }

    #[test]
    fn get_balance_test(){
        let balances: balances::Pallet<String, Balance> = super::Pallet::new();
        assert_eq!(balances.get_balance("Accoun1".to_string()), 0);
    }

    #[test]
    fn transfer_test(){
        let mut balances = super::Pallet::new();
        balances.set_balance(&"Account1".to_string(), 10);
        balances.set_balance(&"Account2".to_string(), 10);
       
        assert_eq!(balances.transfer("Account1".to_string(), "Account2".to_string(), 5), Ok(()));
    }

    #[test]
    fn transfer_insuficient_balance_test(){
        let mut balances = super::Pallet::new();
        
        assert_eq!(
            balances.transfer("Account1".to_string(), "Account2".to_string(), 5),
            Err("Inssuficient balance")
        )
    }
}
