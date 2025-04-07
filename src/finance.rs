use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd)]
pub struct Money(pub f64);

impl Add<Self> for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Self> for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0:.2}¢", self.0)
    }
}

pub const STARTING_PROFIT_AMOUNT: Money = Money(5.0);
pub const BANKRUPTCY_THRESHOLD: Money = Money(0.0);

#[derive(Debug)]
pub struct Finance {
    pub capital: Money,
    pub expenses: Money,
}

impl Display for Finance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}/{1}", self.capital, self.expenses)
    }
}

impl Default for Finance {
    fn default() -> Self {
        Self {
            capital: STARTING_PROFIT_AMOUNT,
            expenses: Money(0.0),
        }
    }
}
