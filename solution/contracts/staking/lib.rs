#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    // imports from openbrush
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };
    // imports from the Staking impl
    use dapp::impls::staking::Data as StakingData;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Staking {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field] // <- add this line
        staking: StakingData, // <- add this line
    }

    // contains default implementation without any modifications
    impl PSP22 for Staking {}

    // contains the extended logic to the PSP22 contract
    impl dapp::Staking for Staking {}

    impl Staking {
        /// Mint a fixed supply of 42_000_000 Staking tokens to be used for Voting.
        ///
        /// Tokens are issued to the account instantiating this contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
                ._mint_to(instance.env().caller(), 42_000_000 * 10u128.pow(18))
                .expect("Should mint");
            instance
        }
    }
}
