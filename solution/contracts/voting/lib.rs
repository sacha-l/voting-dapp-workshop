#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod voting {
    
  // Imports from openbrush
    use openbrush::traits::Storage;
    use dapp::impls::voting::{
            Data as VotingData,
            *,
        };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct VotingContract {
        #[storage_field]
        voting: VotingData,
    }
    
    impl Voting for VotingContract {}

    impl VotingContract {
        #[ink(constructor)]
        pub fn new(my_psp22_account: AccountId) -> Self {
            let mut instance = Self::default();
            instance.voting.my_psp22 = my_psp22_account;
            instance
        }
    }
}
