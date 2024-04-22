use std::collections::BTreeMap;

type AccountId = String; 
type Balance = u128;

#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<AccountId, Balance>
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, account: String, amount: u128) {
        self.balances.insert(account, amount);
    }

    pub fn get_balance(&self, account: String) -> u128 {
        *self.balances.get(&account).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: String, to: String, amount: u128)-> Result<(), &'static str> {
        if self.get_balance(from.clone()) < amount {
            return Err("Inssuficient balance");
        }
        self.set_balance(from.clone(), self.get_balance(from) - amount);
        self.set_balance(to.clone(), self.get_balance(to) + amount);
        
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn set_balance_test() {
        let mut balances = super::Pallet::new();
        balances.set_balance("Account1".to_string(), 10);
        assert_eq!(balances.get_balance("Account1".to_string()), 10);
    }

    #[test]
    fn get_balance_test(){
        let balances = super::Pallet::new();
        assert_eq!(balances.get_balance("Accoun1".to_string()), 0);
    }

    #[test]
    fn transfer_test(){
        let mut balances = super::Pallet::new();
        balances.set_balance("Account1".to_string(), 10);
        balances.set_balance("Account2".to_string(), 10);
       
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
