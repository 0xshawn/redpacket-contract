#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::new_without_default)]

use ink_lang as ink;

#[ink::contract]
mod redpacket {
    #[ink(storage)]
    pub struct RedPacket {}

    impl RedPacket {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_red_packet(&mut self) {
            let value = 9;
            assert!(value <= self.env().balance(), "insufficient funds!");
            assert!(value <= 1, "insufficient funds!");

            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "RedPacket request failed. this can be the case if the contract does not\
                     have sufficient free funds or if the transfer would have brought the\
                     contract's balance below minimum balance."
                )
            }
        }

        #[ink(message)]
        pub fn get_balance(&mut self) -> Balance {
            return self.env().balance();
        }
    }
}
