use openbrush::{
    contracts::psp22::PSP22Error,
    traits::String,
};
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StakingErr {
    PSP22Error(PSP22Error),
    LockingPeriodNotEnded,
    AmountMustBeAboveZero,
    NothingToWithdraw,
    NoVotingPower,
    ProposalDoesNotExist,
    IncorrectOption,
    ProposalExpired,
    AtLeastTwoOptions,
    MaxFourOptions,
    AtLeastOneDay,
}

impl From<StakingErr> for PSP22Error {
    fn from(err: StakingErr) -> Self {
        match err {
            StakingErr::PSP22Error(err) => err,
            _ => PSP22Error::Custom(String::from("Custom")),
        }
    }
}

impl From<PSP22Error> for StakingErr {
    fn from(err: PSP22Error) -> Self {
        StakingErr::PSP22Error(err)
    }
}
