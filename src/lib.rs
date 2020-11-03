use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen};

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are

#[derive(Default, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Badge {
    pub name: String,
    pub url: String,
    pub owner: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct BadgeCollection {
    pub badges: HashMap<String, Badge>,
    pub owners: HashMap<String, Vec<String>>,
    pub collectors: HashMap<String, Vec<String>>,
}

#[near_bindgen]
impl BadgeCollection {
    pub fn create_badge(&mut self, name: String, url: String) -> bool {
        match self.badges.get_mut(&name.clone()) {
            Some(_) => {
                return false;
            }
            None => {
                let owner = env::signer_account_id();
                let badge = Badge {
                    name: name.clone(),
                    url: url.clone(),
                    owner: owner.clone(),
                };

                self.badges.insert(name.clone(), badge.clone());
                match self.owners.get_mut(&owner.clone()) {
                    Some(badges) => {
                        badges.push(name);
                    }
                    None => {
                        self.owners.insert(owner.clone(), vec![name.clone()]);
                    }
                }
            }
        }
        return true;
    }

    pub fn reward_badge(&mut self, name: String, collector: String) -> bool {
        match self.badges.get_mut(&name.clone()) {
            Some(badge) => {
                let owner = env::signer_account_id();

                if badge.owner != owner {
                    return false;
                }

                match self.collectors.get_mut(&collector) {
                    Some(badges) => {
                        if !badges.contains(&name) {
                            badges.push(name.clone());
                        }
                    }
                    None => {
                        self.collectors.insert(collector, vec![name]);
                    }
                }
            }
            None => return false,
        }
        return true;
    }

    pub fn get_collected_badges(&self, collector: ValidAccountId) -> Vec<Badge> {
        env::log(format!("{}", collector.as_ref()).as_bytes());
        let mut collected = Vec::new();
        match self.collectors.get(collector.as_ref()) {
            Some(badges) => {
                for badge in badges {
                    env::log(format!("{}", badge).as_bytes());
                    collected.push(self.badges.get(badge).unwrap().clone());
                }
            }
            None => {}
        }
        collected
    }

    pub fn get_created_badges(&self, owner: ValidAccountId) -> Vec<Badge> {
        env::log(format!("{}", owner.as_ref()).as_bytes());
        let mut created = Vec::new();
        match self.owners.get(owner.as_ref()) {
            Some(badges) => {
                for badge in badges {
                    env::log(format!("{}", badge).as_bytes());
                    created.push(self.badges.get(badge).unwrap().clone());
                }
            }
            None => {}
        }
        created
    }
}
