#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod voting {
    // Imports from openbrush
    use dapp::impls::voting::{
        Data as VotingData,
        *,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct VotingContract {
        #[storage_field]
        voting: VotingData,
    }

    impl Voting for VotingContract {}

    impl VotingContract {
        /// Instantiate this contract using the contract address of the Staking contract.
        ///
        /// In order to vote on a proposal, users will need to have tokens staked.
        #[ink(constructor)]
        pub fn new(my_psp22_account: AccountId) -> Self {
            Self {
                voting: VotingData::new(my_psp22_account),
            }
        }
    }
}
