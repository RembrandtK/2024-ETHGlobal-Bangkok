//! WIP changing from counter example to vote contract.

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{alloy_primitives::*, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Votes {
        uint256 number;
        mapping(uint256 => uint256[]) votes;
    }
}

// #[storage]
// #[entrypoint]
// pub struct Votes {
//     number: StorageU256,
//     // owner: StorageAddress,
//     votes: StorageMap<VoterId, StorageVec<StorageU256>>,
// }

pub type RankIndex = u32;
pub type CandidateId = U256;
pub type VoterId = U256;

#[public]
impl Votes {
    pub fn dummy(&self) -> U256 {
        U256::from(0)
    }

    pub fn get_count(&self, voter: VoterId) -> RankIndex {
        self.votes.get(voter).len() as u32
    }

    pub fn get_candidate(&self, voter: VoterId, rank: RankIndex) -> CandidateId {
        self.votes.get(voter).get(rank).unwrap_or_default()
    }

    pub fn push_candidate(&mut self, voter: VoterId, candidate: CandidateId) {
        self.votes.setter(voter).push(candidate);
    }

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
}
