//! WIP changing from counter example to vote contract.

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

// sol_storage! {
//     #[entrypoint]
//     pub struct Votes {
//         uint256 number;
//     }
// }

#[storage]
pub struct Votes {
    number: StorageU256,
    // owner: StorageAddress,
    votes: StorageMap<U256, StorageVec<StorageU256>>,
}

#[public]
impl Votes {
    pub fn dummy(&self) -> U256 {
        U256::from(0)
    }

    // pub fn rank_len(&self) -> U256 {
    //     self.votes.load().len()
    // }

    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    // pub fn get_vote(&self, voter: U256) -> RustcDecodable<StorageVec<StorageU256>, RustcEncodable> {
    //     self.votes.get(voter).into()
    // }

    // pub fn get_vote_len(&self, voter: U256) -> U256 {
    //     self.votes.get(voter)
    // }

    // /// Sets a number in storage to a user-specified value.
    // pub fn set_number(&mut self, new_number: U256) {
    //     self.number.set(new_number);
    // }

    // /// Sets a number in storage to a user-specified value.
    // pub fn mul_number(&mut self, new_number: U256) {
    //     self.number.set(new_number * self.number.get());
    // }

    // /// Sets a number in storage to a user-specified value.
    // pub fn add_number(&mut self, new_number: U256) {
    //     self.number.set(new_number + self.number.get());
    // }

    // /// Increments `number` and updates its value in storage.
    // pub fn increment(&mut self) {
    //     let number = self.number.get();
    //     self.set_number(number + U256::from(1));
    // }
}
