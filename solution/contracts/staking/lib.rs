#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {

    // Imports from openbrush
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };
    // Imports from the Staking impl
    use dapp::impls::staking::Data as StakingData;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Staking {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field] // <- add this line
        staking: StakingData, // <- add this line
    }

    // Section contains default implementation without any modifications
    impl PSP22 for Staking {}

    impl dapp::Staking for Staking {}

    impl Staking {
        /// Mint a fixed supply of 42_000_000 Staking tokens to be used for Voting.
        ///
        /// Tokens are issued to the account instantiating this contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
            _instance
                ._mint_to(_instance.env().caller(), 42_000_000 * 10u128.pow(18))
                .expect("Should mint");
            _instance
        }
    }
}