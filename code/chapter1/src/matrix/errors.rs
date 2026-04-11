use std::{convert::Infallible, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to build matrix: {}", match .0 {
    BuildErrorKind::AuxiliaryAlloc => "auxiliary allocation failed"
})]
pub struct BuildError(BuildErrorKind);

impl FromStr for BuildError {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alloc" => Ok(Self(BuildErrorKind::AuxiliaryAlloc)),
            _ => panic!("you got the error wrong"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum BuildErrorKind {
    AuxiliaryAlloc,
}
