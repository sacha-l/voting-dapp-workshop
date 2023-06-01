# Add the Voting trait and its ink!plementation

This section steps through writing a Voting trait and adding its implementation.
Read the code explanation in the panel on the right to understand how we've implemented the voting logic.

1. In the `src/traits/` folder, add the trait definition for Voting by creating `voting.rs` and pasting in the following:

    ```rust
    pub use crate::libs::errors::StakingErr;
    pub use ink::prelude::vec::Vec;
    pub use openbrush::traits::{
        Balance,
        String,
        Timestamp,
    };

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

    impl Data {
        pub fn new(staking: AccountId) -> Self {
            Self {
                proposals: Default::default(),
                last_id: Default::default(),
                staking,
                _reserved: None,
            }
        }
    }
    ```

1. Paste in the boilerplate code from the panel on the right. This adds the modifier we'll use in the next step as well as the boilerplate for writing the `impl` block.
    
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

1. Add the logic for `vote`:

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

#### **💡Code explanation**

In steps 1 and 2, we've added the voting trait for our contract which defines 2 methods:
- `propose`, allowing users to propose a new item to be voted on
- `vote`, allowing users to vote on a proposal

In steps 3-7, we've added the implementation of the `Voting` trait. We've also created the `new` method which is the constructor of our contract and adding relevant error types.

The implementation for `propose` enforces the following:

- A user must input the name of the proposal, e.g. “Proposal 1”
- A user must input a maximum of 4 options and a minimum of 2
- A user must input a duration, i.e. the amount of time that this proposal is valid for

Once all these are checked, we simply write the proposal data to our contract’s storage and increment the `last_id` which we also write to storage.

Things to notice about the `vote` implementation:

- We’re using the `has_voting_power` modifier to check the Staking contract's storage
- A user must call this function with a valid proposal ID and option number

Finally, we updated the `errors.rs` file with the new error enums we introduced in the implementations for `vote` and `propose`.

#### **🔎 Using function modifiers**

Our Voting logic implementation consists of adding the functions `propose` and `vote`. However there’s something to notice that’s quite different in how our Voting contract is designed. Recall the diagram from the previous part of this tutorial:

<p align ="center">
<img src="../assets/voting-logic-diagram.png"  width="400">
</p>

We want the `vote` function to call into our Staking contract so it can check whether the caller has voting power and only allow a user to call `vote` if they have some voting power. To write this check for `vote` we’ll write a special function that uses an OpenBrush macro called `modifiers`. This is just like using function modifiers in Solidity. This attribute ensures that a the `vote` function (annotated with `#[openbrush::modifiers(has_voting_power)]`) has the following execution path: 

- When `vote` is called, the modifier will check if the caller has any voting power by checking the storage of our Staking contract
- If the account’s voting power is `0`, it returns an error
- Otherwise, it continues the execution of the `vote` function, returning the storage of the Voting contract

#### **👷 Boilerplate code**

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
    // Add the implementation for `propose`

    // Add the implementation for `vote`
}
```

#### **✅ Final code for `src/impls/voting.rs`**

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

    impl Data {
        pub fn new(staking: AccountId) -> Self {
            Self {
                proposals: Default::default(),
                last_id: Default::default(),
                staking,
                _reserved: None,
            }
        }
    }

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
    }
```

<!-- tabs:end -->

> **Note:** Your code won't compile as is just yet. Head onto the next section to link the `voting` logic to the dependencies of your project.