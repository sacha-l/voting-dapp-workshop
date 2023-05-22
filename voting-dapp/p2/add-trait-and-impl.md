# Add the Voting trait and its implementation

This section steps you through creating a Voting trait for the Voting contract and adding the trait implementations.
Have a look at the code explanation in the panel on the right for how we've implemented the voting logic.

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

1. In the `src/impls` folder, create a new files called `voting.rs` and start by adding in the imports and storage items for the Voting implementation:

    ```rust
    use crate::traits::staking::StakingRef;
    pub use crate::traits::voting::*;
    use openbrush::{
        storage::Mapping,
        traits::{
            AccountId,
            Storage,
            Timestamp,
        },
    };

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    #[derive(Debug)]
    pub struct Data {
        pub proposals: Mapping<Id, (String, Timestamp, Vec<(String, Balance)>)>,
        pub last_id: Id,
        pub staking: AccountId,
        pub _reserved: Option<()>,
    }
    ```

1. Paste in the following code to add in the modifier we'll use in step 4 and start writing the impl block:

    ```rust
    #[openbrush::modifier_definition]
    fn has_voting_power<R, T, F>(instance: &mut T, body: F) -> Result<R, StakingErr>
    where
        T: Storage<Data>,
        F: FnOnce(&mut T) -> Result<R, StakingErr>,
    {
        if !StakingRef::voting_power(&instance.data().staking, T::env().caller()) == 0 {
            return Err(StakingErr::NoVotingPower)
        }

        body(instance)
    }

    impl<T> Voting for T
    where
        T: Storage<Data>,
    {
        // Add implementation for propose
        // Add implementation for vote
    }
    ```
    
1. Add the logic for `propose`:

    ```rust
    fn propose(&mut self, name: String, options: Vec<String>, duration: Timestamp) -> Result<(), StakingErr> {
            
            let id: u128 = self.data().last_id;

            // user must input at least 2 options 
            if options.len() < 2 {
                return Err(StakingErr::AmountMustBeAboveZero)
            }

            // user must input max 4 options
            if options.len() > 4 {
                return Err(StakingErr::MaxFourOptions)
            }

            // proposal must be up for at least one day
            if duration < 24 * 60 * 60 * 1000 {
                return Err(StakingErr::AtLeastOneDay)
            }

            // write proposal to storage
            self.data().proposals.insert(
                &id,
                &(
                    name,
                    Self::env().block_timestamp() + duration,
                    options.iter().map(|option| (option.clone(), 0)).collect(),
                ),
            );

            // increment last_id
            self.data().last_id += 1;
            
            Ok(())
        }
    ```

1. Add the implementation for `vote`:

    ```rust
        #[openbrush::modifiers(has_voting_power)]
        fn vote(&mut self, proposal_id: Id, option: u8) -> Result<(), StakingErr> {
            // exists
            if let Some((name, expiration, options)) = self.data().proposals.get(&proposal_id) {
                // expired
                if expiration < Self::env().block_timestamp() {
                    return Err(StakingErr::ProposalExpired)
                }
                // correct option
                if option >= options.len() as u8 {
                    return Err(StakingErr::IncorrectOption)
                }

                let mut original = options;
                original[option as usize] = (
                    original[option as usize].0.clone(),
                    original[option as usize].1 + StakingRef::voting_power(&self.data().staking, Self::env().caller()),
                );

                self.data()
                    .proposals
                    .insert(&proposal_id, &(name, expiration, original));
            } else {
                return Err(StakingErr::ProposalDoesNotExist)
            }
            Ok(())
        }
    ```

5. Update `src/libs/errors.rs` with new error types:

    ```rust
    pub enum StakingErr {
        // -- snip --
        NoVotingPower,
        ProposalDoesNotExist,
        IncorrectOption,
        ProposalExpired,
        MaxFourOptions,
        AtLeastOneDay
    }
    ```

<!-- slide:break -->

<!-- tabs:start -->

#### **Using OpenBrush modifiers**

Our Voting logic implementation consists of adding the functions `propose` and `vote`. However there’s something to notice that’s quite different in how our Voting contract is designed. Recall the diagram from the previous part of this tutorial:


<img src="../assets/voting-logic-diagram.png"  width="400">

We want the `vote` function to call into our Staking contract so it can check whether the caller has voting power. To write this check for `vote` we’ll use an OpenBrush macro we haven’t yet used called `modifiers`. This is just like using modifiers in Solidity, which is like adding an attribute to the trait implementation that has the following execution path: 

- When `vote` is called, the modifier will check if the caller has any voting power by checking the storage of our Staking contract
- If the account’s voting power is 0, it returns an error
- Otherwise, it continues execution of the `vote` function, returning the storage of the Voting contract

#### **💡Code explanation**

In steps 1 and 2, we've added the voting trait for our contract which defines 3 methods:
- `propose`, allowing users to propose a new item to be voted on
- `vote`, allowing users to vote on a proposal

In steps 3-7, we've added the implementation of the `Voting` trait.
The implementation for `propose` enforces the following:

- A user must input the name of the proposal, e.g. “Proposal 1”
- A user must input a maximum of 4 options and a minimum of 2
- A user must input a duration, i.e. the amount of time that this proposal is valid till

Once all these are checked, we simply write the proposal data to our contract’s storage and increment the `last_id` which we also write to storage.

Things to notice about the `vote` implementation:

- We’re using the `has_voting_power` modifier to check the Staking contract's storage
- A user must call this function with a proposal ID and an option number

<!-- tabs:end -->