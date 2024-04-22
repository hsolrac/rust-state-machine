use std::collections::BTreeMap;

type AccountId = String; 
type BlockNumber= u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber, 
    nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new()
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn increment_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn increment_nonce(&mut self, account: String) {
        let nonce = self.nonce.get(&account).unwrap_or(&0) + 1;
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut system = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
    }
    #[test]
    fn increment_block_number() {
        let mut system = super::Pallet::new();
        system.increment_block_number();
        assert_eq!(system.block_number(), 1);
    }
    #[test]
    fn increment_nonce() {
        let mut system = super::Pallet::new();
        system.increment_nonce("Account1".to_string());
        assert_eq!(system.nonce.get("Account1").unwrap(), &1);
    }
}
