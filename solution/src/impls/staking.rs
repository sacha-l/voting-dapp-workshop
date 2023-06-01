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

        // safely transfer the amount to the contract's account ID
        self._transfer_from_to(caller, Self::env().account_id(), amount, Vec::new())?;

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

        // to keep things simple, the voting power is just the current amount staked
        return current_amount_staked
    }
}
