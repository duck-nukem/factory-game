use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CardMeta {
    pub title: String,
    pub help_text: String,
    pub delta_profit: f64,
    pub delta_co2: f64,
}

impl Default for CardMeta {
    fn default() -> Self {
        Self {
            title: String::from("Nothing"),
            help_text: String::from("Literally nothing"),
            delta_profit: 0.0,
            delta_co2: 0.0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CardCollection {
    pub cards: Vec<CardMeta>,
}

pub trait Deck {
    fn draw_cards(&mut self, hand_size: usize) -> Vec<CardMeta>;
}

impl Deck for CardCollection {
    fn draw_cards(&mut self, hand_size: usize) -> Vec<CardMeta> {
        self.cards
            .drain(0..hand_size.min(self.cards.len()))
            .collect()
    }
}

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
pub fn load_cards() -> CardCollection {
    toml::from_str(CARDS_SOURCE).unwrap_or(CardCollection { cards: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_draw_an_arbitrary_number_of_cards_from_the_deck() {
        let mut deck = CardCollection {
            cards: vec![CardMeta {
                title: String::from("First"),
                help_text: String::new(),
                delta_profit: 0.0,
                delta_co2: 0.0,
            }],
        };

        let hand = deck.draw_cards(1);

        assert_eq!(hand.first().expect("Unable to draw hand").title, "First")
    }

    #[test]
    fn test_drawing_more_cards_then_available_returns_all_remaining_cards() {
        let mut deck = CardCollection {
            cards: vec![CardMeta {
                title: String::from("First"),
                help_text: String::new(),
                delta_profit: 0.0,
                delta_co2: 0.0,
            }],
        };

        let hand = deck.draw_cards(5);

        assert_eq!(1, hand.len())
    }

    #[test]
    fn test_drawn_cards_are_gone_from_the_deck() {
        let mut deck = CardCollection {
            cards: vec![CardMeta {
                title: String::from("First"),
                help_text: String::new(),
                delta_profit: 0.0,
                delta_co2: 0.0,
            }],
        };

        deck.draw_cards(1);

        assert_eq!(0, deck.cards.len())
    }
}
