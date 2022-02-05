#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::new_without_default)]

use ink_lang as ink;

#[ink::contract]
mod redpacket {
    // use ink_storage::collections::{
    //     HashMap as StorageHashMap,
    // };

    #[ink(storage)]
    pub struct RedPacket {
        // value: u32,
    }

    impl RedPacket {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_red_packet(&mut self, value: Balance) {
            let value = 1;
            assert!(value <= self.env().balance(), "insufficient funds!");

            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "RedPacket request failed. this can be the case if the contract does not\
                     have sufficient free funds or if the transfer would have brought the\
                     contract's balance below minimum balance."
                )
            }
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut redpacket = RedPacket::new(false);
            assert_eq!(redpacket.get(), false);
            redpacket.flip();
            assert_eq!(redpacket.get(), true);
        }
    }
}
