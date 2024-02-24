use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[near_bindgen]
pub struct Antugrow {
    pub near_names: HashMap<String, String>,
    pub farmer_wallets: HashMap<String, String>,
}

impl Default for Antugrow {
    fn default() -> Self {
        Self {
            near_names: HashMap::new(),
            farmer_wallets: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl Antugrow {
    pub fn assign_near_name(&mut self, member_initials: String, group_name: String) -> String {
        let mut new_name = String::new();
        let hash = env::sha256(member_initials.as_bytes());
        log!("hash from member_initials: {:?}", hash);

        if let Some(_existing_name) = self.near_names.get(&member_initials) {
            let unique_identifier = Self::generate_unique_identifier();
            new_name = format!("{}{}.{}.antugrow.near", member_initials, unique_identifier, group_name);
        } else {
            new_name = format!("{}.{}.antugrow.near", member_initials, group_name);
        }
        self.near_names.insert(member_initials.clone(), new_name.clone());
        new_name
    }

        #[handle_result]
        pub fn create_custodial_wallet(&mut self, farmer_near_name: String) -> Result<String, &'static str> {
            if self.farmer_wallets.contains_key(&farmer_near_name) {
                return Err("Wallet already exists for the given farmer name.");
            }

            let wallet_name = format!("{}.custodial", farmer_near_name);
            self.farmer_wallets.insert(farmer_near_name.clone(), wallet_name.clone());
            Ok(wallet_name)
        }
    
    

    pub fn generate_unique_identifier() -> String {
        let current_block_timestamp = env::block_timestamp();
        current_block_timestamp.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, test_utils::VMContextBuilder};

    fn setup_contract() -> Antugrow {
        let context = VMContextBuilder::new()
            .predecessor_account_id("test_user.near".parse().unwrap())
            .build();
        testing_env!(context);
        Antugrow::default()
    }

    #[test]
    fn test_assign_near_name() {
        let mut contract = setup_contract();
        let member_initials = "AB".to_string();
        let group_name = "group1".to_string();
        let result = contract.assign_near_name(member_initials, group_name.clone());
        assert!(result.contains(&group_name));
    }

    #[test]
    fn test_create_custodial_wallet() {
        let mut contract = setup_contract();
        let farmer_near_name = "farmer1.near".to_string();
        let result = contract.create_custodial_wallet(farmer_near_name.clone());
        assert!(result.is_ok());
        assert!(result.unwrap().contains(&farmer_near_name));
    }

    #[test]
    fn test_wallet_exists() {
        let mut contract = setup_contract();
        let farmer_near_name = "farmer1.near".to_string();
        let wallet_name = contract.create_custodial_wallet(farmer_near_name.clone()).unwrap();
        assert!(contract.farmer_wallets.contains_key(&farmer_near_name));

        let non_existent_farmer_name = "non_existent_farmer.near".to_string();
        assert!(!contract.farmer_wallets.contains_key(&non_existent_farmer_name));
    }
}
