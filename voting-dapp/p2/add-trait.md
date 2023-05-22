# Add the Voting trait

1. In the `src/traits/` folder, add the trait definition for Voting by creating `voting.rs` and pasting in the following:

    ```rust
    pub use crate::libs::errors::StakingErr;
    pub use ink::prelude::vec::Vec;
    pub use openbrush::traits::{
        Balance,
        String,
        Timestamp,
    };

    #[openbrush::wrapper]
    pub type VotingRef = dyn Voting;

    pub type Id = u128;

    #[openbrush::trait_definition]
    pub trait Voting {
        #[ink(message)]
        fn propose(&mut self, name: String, options: Vec<String>, duration: Timestamp) -> Result<(), StakingErr>;

        #[ink(message)]
        fn vote(&mut self, proposal_id: Id, option: u8) -> Result<(), StakingErr>;
    }
    ```

1. Update the `src/traits/mod.rs` file:

    ```rust
    pub mod voting;
    ```