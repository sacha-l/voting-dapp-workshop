pub use crate::libs::errors::StakingErr;
pub use openbrush::{
    contracts::psp22::*,
    traits::{
        AccountId,
        Balance,
    },
};

#[openbrush::wrapper]
pub type StakingRef = dyn Staking + PSP22;

#[openbrush::trait_definition]
pub trait Staking {
    #[ink(message)]
    fn stake(&mut self, amount: Balance) -> Result<(), StakingErr>;

    #[ink(message)]
    fn unstake(&mut self, amount: Balance) -> Result<(), StakingErr>;

    #[ink(message)]
    fn voting_power(&self, account: AccountId) -> u128;
}
