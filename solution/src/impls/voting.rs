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
    // this checks if the caller has any voting power (i.e. tokens staked)
    if StakingRef::voting_power(&instance.data().staking, T::env().caller()) < 100 * 10u128.pow(18) {
        return Err(StakingErr::NotEnoughVotingPower)
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
                original[option as usize].1 + 1 as u128, // adds a single vote 
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
