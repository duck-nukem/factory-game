use std::{fmt::Display, ops::Add};

use serde::Deserialize;

pub const CATASTROPHIC_POLLUTION_THRESHOLD: Co2Emission = Co2Emission(20.0);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd)]
pub struct Co2Emission(pub f64);

impl Display for Co2Emission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0} tCO₂e", self.0)
    }
}

impl Add<Self> for Co2Emission {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
