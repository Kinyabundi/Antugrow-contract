// Import the necessary NEAR libraries
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::LookupMap;
use blake2::{Blake2b512, Blake2s256, Digest};

// Define the contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Antugrow {
    pub near_names: LookupMap<String, String>,
    pub farmer_wallets: LookupMap<String, String>,
}

impl Antugrow {
    pub fn new() -> Self {
        Self {
            near_names: LookupMap::new(b"n".to_vec()),
            farmer_wallets: LookupMap::new(b"w".to_vec()),
        }
    }

    pub fn assign_near_name(&mut self, member_initials: String, group_name: String) -> String {
        let hashed_initials = Self::hash_initials(&member_initials);
        let mut new_name = String::new();
        if let Some(_existing_name) = self.near_names.get(&hashed_initials) {
            let unique_identifier = Self::generate_unique_identifier();
            new_name = format!("{}{}.{}.antugrow.near", hashed_initials, unique_identifier, group_name);
        } else {
            new_name = format!("{}.{}.antugrow.near", hashed_initials, group_name);
        }
        self.near_names.insert(&hashed_initials, &new_name);
        new_name
    }

    pub fn create_custodial_wallet(&mut self, farmer_near_name: String) -> String {
        let wallet_name = format!("{}.custodial", farmer_near_name);
        self.farmer_wallets.insert(&farmer_near_name, &wallet_name);
        wallet_name
    }

    pub fn hash_initials(initials: &str) -> String {
        let mut hasher = Blake2b512::new();
        hasher.update(initials);
        let result = hasher.finalize();
        let mut hex_string = String::new();
        for byte in result.as_slice(){
            hex_string.push_str(&format!("{:02x}", byte))
        }
        hex_string
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
    // use near_sdk::json_types::ValidAccountId;

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