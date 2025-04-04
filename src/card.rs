use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CardMeta {
    pub title: String,
    pub help_text: String,
    pub delta_profit: f64,
    pub delta_co2: f64,
}

#[derive(Debug, Deserialize)]
pub struct CardCollection {
    pub cards: Vec<CardMeta>,
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

const CARDS_SOURCE: &str = include_str!("../resources/cards.toml");

#[must_use]
pub fn load_cards() -> Vec<CardMeta> {
    let collection: CardCollection =
        toml::from_str(CARDS_SOURCE).unwrap_or(CardCollection { cards: vec![] });

    collection.cards
}
