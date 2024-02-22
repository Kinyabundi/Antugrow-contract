// Import the necessary NEAR libraries
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;
use near_sdk::serde::{Serialize, Deserialize};

// Define the contract
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[near_bindgen]
pub struct Antugrow {
    pub near_names: HashMap<String, String>,
    pub farmer_wallets: HashMap<String, String>,
}
// Implement the default constructor for the contract
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
    pub fn new() -> Self {
        Self {
            near_names: HashMap::new(),
            farmer_wallets: HashMap::new(),
        }
    }

    pub fn assign_near_name(&mut self, member_initials: String, group_name: String) -> String {
        let mut new_name = String::new();
        if let Some(_existing_name) = self.near_names.get(&member_initials) {
            let unique_identifier = Self::generate_unique_identifier();
            new_name = format!("{}{}.{}.antugrow.near", member_initials, unique_identifier, group_name);
        } else {
            new_name = format!("{}.{}.antugrow.near", member_initials, group_name);
        }
        self.near_names.insert(member_initials.clone(), new_name.clone());
        new_name
    }

    pub fn create_custodial_wallet(&mut self, farmer_near_name: String) -> String {
        let wallet_name = format!("{}.custodial", farmer_near_name);
        self.farmer_wallets.insert(farmer_near_name.clone(), wallet_name.clone());
        wallet_name
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
        Antugrow::new()
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
        assert!(result.contains(&farmer_near_name));
    }
}
