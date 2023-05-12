# Implement the Staking trait

Continuing with the Rusty pattern we know and love, now that we have our trait definitions all that’s left is to write our trait’s ink!plementations (*that’s probably the word squink! would use after all* 😉).

1. In your project’s `src/impls/` folder, create a new file called `staking.rs` and copy the scaffold code in the right panel:

1. Update your project by updating the `src/impls/mod.rs` file with: 

    ```rust
    pub mod staking;
    ```

1. You'll need a way to keep track of what’s been staked, which we'll do using the common ink! pattern for declaring storage. Replace the `// Storage goes here` line with the following code snippet: 

```rust
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub staked: Mapping<AccountId, (Balance, Timestamp)>,
    pub _reserved: Option<()>,
}
```

**Add the trait ink!plementations**

Now let’s get to the ink!plementation. The following steps don’t go into too much detail about specific implementation decisions — make sure to read the code comments and the Code Explanation in the side panel to understand the logic.

1. Replace the comment that says `//Staking trait impl block goes here` with the following code block for `stake`
    
    ```rust
    fn stake(&mut self, amount: Balance) -> Result<(), StakingErr> {
            // get the AccountId of this caller
            let caller = Self::env().caller();
    
            // get the staking data
            let staked = &self.data::<Data>().staked.get(&caller);
    
            // if the amount is 0, then return an error
            if amount == 0 {
                return Err(StakingErr::AmountMustBeAboveZero)
            }
    
            // if the caller has some tokens already staked, accumulate the amount
            if let Some(staking_data) = staked {
                // calculate accumulated stake
                let accumulated_amount = staking_data.0 + amount;
                // update contract storage with accumulated stake
                let _ = &self
                    .data::<Data>()
                    .staked
                    .insert(&caller, &(accumulated_amount, staking_data.1));
            } else {
                // otherwise, insert the amount to the staking data
                let _ = &self
                    .data::<Data>()
                    .staked
                    .insert(&caller, &(amount, Self::env().block_timestamp()));
            }
    
            // safely transfer the amount to the contract's account ID
    				PSP22Ref::transfer_from_builder(
                &Self::env().account_id(),
                caller,
                Self::env().account_id(),
                amount,
                Vec::<u8>::new(),
            )
            .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
            .invoke()?;
    				
            Ok(())
        }
    ```
    
2. Add in the following code for `unstake`:
    
    ```rust
    fn unstake(&mut self, amount: Balance) -> Result<(), StakingErr> {
            // get the AccountId of this caller
            let caller = Self::env().caller();
    
            // get the staking data
            let staked = &self.data::<Data>().staked.get(&caller);
    
            // users must enter an amount greater than zero
            if amount == 0 {
                return Err(StakingErr::AmountMustBeAboveZero)
            }
    
            // used to calculated if locking period has expired based on 30 days in Unix time
            const UNIX_DAY: u64 = 86400;
            let month: u64 = 30 * UNIX_DAY;
    
            if let Some(staking_data) = staked {
                // get current staked balance
                let current_stake = staking_data.0;
    
                // if user input is equal to or more than their current stake
                if amount >= current_stake {
    
                    // first check if they are allowed to unstake
                    if Self::env().block_timestamp() - staking_data.1 < month {
                        return Err(StakingErr::LockingPeriodNotEnded)
                    }
    
                    // return all staked tokens back to the caller
                    self.transfer_from(Self::env().account_id(), caller, current_stake, Vec::default())?;
    
                    // clean up storage by removing staking data for this caller
                    self.data().staked.remove(&caller);
    
                } else {
                    // otherwise, update the staked amount and reset staking timestamp
                    self.data()
                        .staked
                        .insert(&caller, &(current_stake - amount, Self::env().block_timestamp()));
    
                    // and transfer the amount to be unstaked back to the caller
                    self.transfer_from(Self::env().account_id(), caller, amount, Vec::default())?;
                }
            } else {
                // this means that the stake is None
                return Err(StakingErr::NothingToWithdraw)
            }
    
            Ok(())
        }
    ```
    
3. Finally, add this code snippet for `voting_power`:
    
    ```rust
    fn voting_power(&self, account: AccountId) -> u128 {
            // get the amount the account ID has staked
            let staked = &self.data::<Data>().staked.get(&account);
    
            // use Internal trait to get voting power
            if let Some(staking_data) = staked {
                return self._calculate_voting_power(&staking_data)
            } else {
                0
            }
        }
    
    ```
    
4. All that’s left to do now is to write our helper function, `_calculate_voting_power` we’ve used above. Replace the `// Internal helpers go here` line with the following code snippet:
    
    ```rust
    // Internal helpers for the Staking trait implementation
    pub trait Internal {
        /// Calculates voting power based on amount staked and time locked
        fn _calculate_voting_power(&self, staking_data: &(Balance, Timestamp)) -> u128;
    }
    
    impl<T> Internal for T
    where
        T: Storage<Data>,
    {
        fn _calculate_voting_power(&self, staking_data: &(Balance, Timestamp)) -> u128 {
            
            // get the current amount staked
            let current_amount_staked = staking_data.0;
    
            // calculate the amount of time passed since the last stake
            let time_until_unlocking = Self::env().block_timestamp() - staking_data.1;
    
            // calculate percentage of time passed until user can unlock
            const UNIX_MONTH: u64 = 2592000; // based on 30 days in Unix time
            let decay_coefficient = 1 - (time_until_unlocking / UNIX_MONTH);
    
            // calculate voting_power using linear decay based on amount staked
    		let voting_power = decay_coefficient as u128 * current_amount_staked;
    
            return voting_power as u128
        }
    }
    ```

What you've accomplished in this section is adding the implementation. But you're contract won't compile as-is: you need to complete the next section, where you will link this implementation back to your contract file.    
    
<!-- slide:break -->

<!-- tabs:start -->

#### **💡Code explanation**

* The Staking contract data will track the amount an account has staked in our dApp, as well as a Timestamp indicating when the stake was made. 
* The helper function is calculating voting power as a function the amount a user has staked and the the amount of time left a user has until they can unstake.
* Wondering how the contract locks and unlocks these tokens? Here’s where the PSP22 interface comes in handy. We’ll use the `transfer_from` function to transfer the staking amount from the caller’s account to the contract’s account ID. For unstaking, we can use the same function but the other way around.*

#### **👷 Scaffold code**

Use this for step 1 in `src/impls/staking.rs`:

```rust
pub use crate::traits::staking::*;
pub use ink::prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        Storage, Timestamp
    },
};

// Storage goes here 

impl<T> Staking for T
where
    T: Storage<Data>,
    T: PSP22,
    T: psp22::Internal,
{
		// Staking trait impl block goes here
}

// Internal helpers go here
```

#### **✅ Final code for `src/impls/staking.rs`**

```rust
pub use crate::traits::staking::*;
pub use ink::prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        Storage,
        Timestamp,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub staked: Mapping<AccountId, (Balance, Timestamp)>,
    pub _reserved: Option<()>,
}

impl<T> Staking for T
where
    T: Storage<Data>,
    T: PSP22,
    T: psp22::Internal,
{
    fn stake(&mut self, amount: Balance) -> Result<(), StakingErr> {
        // get the AccountId of this caller
        let caller = Self::env().caller();

        // get the staking data
        let staked = &self.data::<Data>().staked.get(&caller);

        // if the amount is 0, then return an error
        if amount == 0 {
            return Err(StakingErr::AmountMustBeAboveZero)
        }

        // if the caller has some tokens already staked, accumulate the amount
        if let Some(staking_data) = staked {
            // calculate accumulated stake
            let accumulated_amount = staking_data.0 + amount;
            // update contract storage with accumulated stake
            let _ = &self
                .data::<Data>()
                .staked
                .insert(&caller, &(accumulated_amount, staking_data.1));
        } else {
            // otherwise, insert the amount to the staking data
            let _ = &self
                .data::<Data>()
                .staked
                .insert(&caller, &(amount, Self::env().block_timestamp()));
        }

        // transfer the amount to the contract's account ID
        PSP22Ref::transfer_from_builder(
            &Self::env().account_id(),
            caller,
            Self::env().account_id(),
            amount,
            Vec::<u8>::new(),
        )
        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
        .invoke()?;

        Ok(())
    }

    fn unstake(&mut self, amount: Balance) -> Result<(), StakingErr> {
        // get the AccountId of this caller
        let caller = Self::env().caller();

        // get the staking data
        let staked = &self.data::<Data>().staked.get(&caller);

        // users must enter an amount greater than zero
        if amount == 0 {
            return Err(StakingErr::AmountMustBeAboveZero)
        }

        // used to calculated if locking period has expired based on 30 days in Unix time
        const UNIX_DAY: u64 = 86400;
        let month: u64 = 30 * UNIX_DAY;

        if let Some(staking_data) = staked {
            // get current staked balance
            let current_stake = staking_data.0;

            // if user input is equal to or more than their current stake
            if amount >= current_stake {
                // first check if they are allowed to unstake
                if Self::env().block_timestamp() - staking_data.1 < month {
                    return Err(StakingErr::LockingPeriodNotEnded)
                }

                // return all staked tokens back to the caller
                self.transfer_from(Self::env().account_id(), caller, current_stake, Vec::default())?;

                // clean up storage by removing staking data for this caller
                self.data().staked.remove(&caller);
            } else {
                // otherwise, update the staked amount and reset staking timestamp
                self.data()
                    .staked
                    .insert(&caller, &(current_stake - amount, Self::env().block_timestamp()));

                // and transfer the amount to be unstaked back to the caller
                self.transfer_from(Self::env().account_id(), caller, amount, Vec::default())?;
            }
        } else {
            // this means that the stake is None
            return Err(StakingErr::NothingToWithdraw)
        }

        Ok(())
    }

    fn voting_power(&self, account: AccountId) -> u128 {
        // get the amount the account ID has staked
        let staked = &self.data::<Data>().staked.get(&account);

        // use Internal trait to get voting power
        if let Some(staking_data) = staked {
            return self._calculate_voting_power(&staking_data)
        } else {
            0
        }
    }
}

// Internal helpers for the Staking trait implementation
pub trait Internal {
    /// Calculates voting power as a function of amount staked and time locked
    fn _calculate_voting_power(&self, staking_data: &(Balance, Timestamp)) -> u128;
}

impl<T> Internal for T
where
    T: Storage<Data>,
{
    fn _calculate_voting_power(&self, staking_data: &(Balance, Timestamp)) -> u128 {
        // get the current amount staked
        let current_amount_staked = staking_data.0;

        // calculate the amount of time passed since the last stake
        let time_until_unlocking = Self::env().block_timestamp() - staking_data.1;

        // calculate percentage of time passed until user can unlock
        const UNIX_MONTH: u64 = 2592000; // based on 30 days in Unix time
        let decay_coefficient = 1 - (time_until_unlocking / UNIX_MONTH);

        // calculate voting_power using linear decay based on amount staked
        let voting_power = decay_coefficient as u128 * current_amount_staked;

        return voting_power as u128
    }
}
```
<!-- tabs:end -->

> **Note:** If you're using Rust analyzer you may be seing some errors come up: at this point, we still need to link all of our project's dependencies. Wait until completing the next part to run `cargo check`.