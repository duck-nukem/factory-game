use std::fmt::Display;

use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::{emission::Co2Emission, finance::Money};

#[derive(Clone, Debug, Deserialize)]
pub struct Card {
    pub title: String,
    pub delta_profit: Money,
    pub delta_co2: Co2Emission,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            title: String::from("Nothing"),
            delta_profit: Money(0.0),
            delta_co2: Co2Emission(0.0),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub const fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn pick_card(&self, card_index: usize) -> Option<&Card> {
        self.cards.get(card_index)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards_display = self
            .cards
            .iter()
            .enumerate()
            .map(|(index, card)| format!("{index} -> {card}\n"))
            .reduce(|all_cards, card| all_cards + &card)
            .unwrap_or_else(|| "No cards".to_string());
        write!(f, "{cards_display}")
    }
}

impl Deck {
    pub fn draw_cards(&self, hand_size: usize) -> Vec<Card> {
        let mut deck = self.cards.clone();
        deck.shuffle(&mut rand::rng());
        deck.clone().into_iter().take(hand_size).collect()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0} - {1}, {2}",
            self.title, self.delta_profit, self.delta_co2
        )
    }
}

const CARDS_SOURCE: &str = include_str!("../resources/cards.toml");

#[must_use]
pub fn load_cards() -> Deck {
    toml::from_str(CARDS_SOURCE).unwrap_or(Deck { cards: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_draw_an_arbitrary_number_of_cards_from_the_deck() {
        let deck = Deck {
            cards: vec![Card {
                title: String::from("First"),
                delta_profit: Money(0.0),
                delta_co2: Co2Emission(0.0),
            }],
        };

        let hand = deck.draw_cards(1);

        assert_eq!(hand.first().expect("Unable to draw hand").title, "First")
    }

    #[test]
    fn test_drawing_more_cards_then_available_returns_all_remaining_cards() {
        let deck = Deck {
            cards: vec![Card::default()],
        };

        let hand = deck.draw_cards(5);

        assert_eq!(1, hand.len())
    }
}
