use core::fmt::Display;
use revm::wiring::result::{EVMError, HaltReason, InvalidTransaction};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptimismHaltReason {
    Base(HaltReason),
    FailedDeposit,
}

impl From<HaltReason> for OptimismHaltReason {
    fn from(value: HaltReason) -> Self {
        Self::Base(value)
    }
}
