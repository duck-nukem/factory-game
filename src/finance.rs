use std::fmt::Display;

pub const STARTING_PROFIT_AMOUNT: f64 = 5.0;
pub const BANKRUPTCY_THRESHOLD: f64 = 0.0;

#[derive(Debug)]
pub struct Finance {
    pub capital: f64,
    pub expenses: f64,
}

impl Display for Finance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0:.2}¢/{1:.2}¢", self.capital, self.expenses)
    }
}

impl Default for Finance {
    fn default() -> Self {
        Self {
            capital: STARTING_PROFIT_AMOUNT,
            expenses: 0.0,
        }
    }
}
