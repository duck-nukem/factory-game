use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct CardMeta {
    pub title: String,
    pub help_text: String,
    pub delta_profit: f64,
    pub delta_co2: f64,
}

pub const DEFAULT_CARD_META: &CardMeta = &CardMeta {
    title: String::new(),
    help_text: String::new(),
    delta_profit: 0.0,
    delta_co2: 0.0,
};

impl Display for CardMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0} - {1}¢, {2} CO₂",
            self.title, self.delta_profit, self.delta_co2
        )
    }
}

#[must_use]
pub fn generate_random_cards() -> Vec<CardMeta> {
    vec![
        CardMeta {
            title: String::from("Hire cheap workers"),
            help_text: String::new(),
            delta_profit: 25.0,
            delta_co2: 50.0,
        },
        CardMeta {
            title: String::from("Buy CO2 offset"),
            help_text: String::new(),
            delta_profit: -10.0,
            delta_co2: -3.0,
        },
    ]
}
