use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>
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

    pub fn balance(&self, account: String) -> u128 {
        *self.balances.get(&account).unwrap_or(&0)
    }
}

