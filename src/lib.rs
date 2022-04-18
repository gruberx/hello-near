use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Gatekeeper {
    greeting: String
    // SETUP CONTRACT STATE
}

#[near_bindgen]
impl Gatekeeper {
    #[init]
    pub fn set_default_greeting() -> Self {
        Self{
            greeting: "Hello ".to_string(),
        }
    }

    pub fn set_greeting(&mut self, wish: String) {
        self.greeting = wish + " ";
    }

    pub fn receive_and_give(&self, name: String) -> String {
        return self.greeting.clone() + &name + "!";
    }
    // ADD CONTRACT METHODS HERE
}