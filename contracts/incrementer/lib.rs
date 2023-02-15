#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        // Store a number
        number: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor 
            Self {
                number: init_value,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                number: 0,
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.number
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.number += by;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
            fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }
        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }
    }
}