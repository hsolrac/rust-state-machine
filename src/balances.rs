use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

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
        self.balances.insert(account.clone(), amount);
    }

    pub fn get_balance(&self, account: &AccountId) -> Balance {
        *self.balances.get(account).unwrap_or(&Balance::zero())
    }

    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance)-> Result<(), &'static str> {
        let caller_balance = self.get_balance(&from);
        let to_balance = self.get_balance(&to);

        if caller_balance < amount {
            return Err("Inssuficient balance");
        }

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(from, new_caller_balance);
        self.balances.insert(to, new_to_balance);
        
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn set_balance_test() {
        let mut balances = super::Pallet::new();
        balances.set_balance(&"Account1".to_string(), 10);
        assert_eq!(balances.get_balance(&"Account1".to_string()), 10);
    }

    #[test]
    fn get_balance_test(){
        let balances = super::Pallet::<String, u128>::new();
        assert_eq!(balances.get_balance(&"Accoun1".to_string()), 0);
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
        let mut balances = super::Pallet::<String, u128>::new();
        
        assert_eq!(
            balances.transfer("Account1".to_string(), "Account2".to_string(), 5),
            Err("Inssuficient balance")
        )
    }
}
