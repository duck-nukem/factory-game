use std::fmt::Display;

use crate::{
    card::{Card, Deck, Hand, load_cards},
    emission::{CATASTROPHIC_POLLUTION_THRESHOLD, Co2Emission},
    finance::{BANKRUPTCY_THRESHOLD, Finance, Money},
    math::exponential_curve,
};

const ROUNDS_TO_BEAT_THE_GAME: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub enum PlaythroughStatus {
    Ongoing,
    GameOver,
    Beaten,
}

#[derive(Debug)]
pub struct GameState {
    finance: Finance,
    accumulated_co2_emission: Co2Emission,
    deck: Deck,
    played_cards: Vec<Card>,
    pub hand: Hand,
    pub playthrough_status: PlaythroughStatus,
}

impl GameState {
    pub fn get_round(&self) -> i32 {
        self.played_cards.len().try_into().unwrap_or_default()
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0} | {1}/{4} | Round {2}/{3}",
            self.finance,
            self.accumulated_co2_emission,
            self.played_cards.len(),
            &ROUNDS_TO_BEAT_THE_GAME,
            &CATASTROPHIC_POLLUTION_THRESHOLD,
        )
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            finance: Finance::default(),
            accumulated_co2_emission: Co2Emission::default(),
            played_cards: vec![],
            deck: load_cards(),
            hand: Hand::default(),
            playthrough_status: PlaythroughStatus::Ongoing,
        }
    }
}

pub enum Action {
    GainMoney(Money),
    SetExactAmount(Money),
    IncreaseCo2Emission(Co2Emission),
    SetExactCo2Emission(Co2Emission),
    PlayCard(Card),
    DrawCards(usize),
}

#[must_use]
pub fn game_state_reducer(state: GameState, action: Action) -> GameState {
    match action {
        Action::GainMoney(incoming_amount) => GameState {
            finance: Finance {
                capital: state.finance.capital + incoming_amount,
                ..state.finance
            },
            ..state
        },
        Action::SetExactAmount(amount) => GameState {
            finance: Finance {
                capital: amount,
                ..state.finance
            },
            ..state
        },
        Action::IncreaseCo2Emission(co2_emission) => {
            let accumulated_co2_emission = state.accumulated_co2_emission + co2_emission;

            if accumulated_co2_emission < Co2Emission(0.0) {
                return state;
            }

            GameState {
                accumulated_co2_emission,
                ..state
            }
        }
        Action::SetExactCo2Emission(co2_emission) => GameState {
            accumulated_co2_emission: co2_emission,
            ..state
        },
        Action::DrawCards(hand_size) => GameState {
            hand: Hand::new(state.deck.draw_cards(hand_size)),
            ..state
        },
        Action::PlayCard(card) => {
            let accrued_profit = state.finance.capital + card.delta_profit;
            let mut accumulated_co2_emission = state.accumulated_co2_emission + card.delta_co2;
            let played_cards: Vec<Card> =
                state.played_cards.into_iter().chain(vec![card]).collect();

            if accumulated_co2_emission < Co2Emission(0.0) {
                accumulated_co2_emission = Co2Emission::default();
            }

            let is_bankrupt = accrued_profit < BANKRUPTCY_THRESHOLD;
            let has_failed_to_meet_profit_target = accrued_profit < state.finance.expenses;
            let is_pollution_catastrophic =
                accumulated_co2_emission > CATASTROPHIC_POLLUTION_THRESHOLD;
            let has_player_completed_all_required_levels =
                played_cards.len() > ROUNDS_TO_BEAT_THE_GAME;

            let playthrough_status =
                if is_bankrupt || has_failed_to_meet_profit_target || is_pollution_catastrophic {
                    PlaythroughStatus::GameOver
                } else if has_player_completed_all_required_levels {
                    PlaythroughStatus::Beaten
                } else {
                    state.playthrough_status
                };
            let round = u32::try_from(played_cards.len()).map_or_else(|_| 32.0, f64::from);

            GameState {
                finance: Finance {
                    capital: accrued_profit,
                    expenses: Money(exponential_curve(0.8, 0.2, round)),
                },
                accumulated_co2_emission,
                played_cards,
                playthrough_status,
                ..state
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        card::{Card, Deck, Hand},
        finance::{BANKRUPTCY_THRESHOLD, STARTING_PROFIT_AMOUNT},
    };

    use super::*;

    #[test]
    fn test_can_acquire_profit() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::GainMoney(Money(1.0)));

        assert_eq!(STARTING_PROFIT_AMOUNT + Money(1.0), state.finance.capital);
    }

    #[test]
    fn test_can_acquire_negative_profit() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::GainMoney(Money(-1.0)));

        assert_eq!(STARTING_PROFIT_AMOUNT - Money(1.0), state.finance.capital);
    }

    #[test]
    fn test_can_set_profit_to_any_value() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::SetExactAmount(Money(1337.0)));

        assert_eq!(Money(1337.0), state.finance.capital);
    }

    #[test]
    fn test_can_increase_co2_emission() {
        let initial_state = GameState::default();

        let state =
            game_state_reducer(initial_state, Action::IncreaseCo2Emission(Co2Emission(1.0)));

        assert_eq!(Co2Emission(1.0), state.accumulated_co2_emission);
    }

    #[test]
    fn test_can_reduce_co2_emission() {
        let initial_state = GameState::default();
        let intermittent_state =
            game_state_reducer(initial_state, Action::SetExactCo2Emission(Co2Emission(5.0)));

        let state = game_state_reducer(
            intermittent_state,
            Action::IncreaseCo2Emission(Co2Emission(-1.0)),
        );

        assert_eq!(Co2Emission(4.0), state.accumulated_co2_emission);
    }

    #[test]
    fn test_co2_emission_cannot_be_negative() {
        let initial_state = GameState::default();

        let state = game_state_reducer(
            initial_state,
            Action::IncreaseCo2Emission(Co2Emission(-1.0)),
        );

        assert_eq!(Co2Emission(0.0), state.accumulated_co2_emission);
    }

    #[test]
    fn test_can_set_co2_emission_to_any_value() {
        let initial_state = GameState::default();

        let state = game_state_reducer(
            initial_state,
            Action::SetExactCo2Emission(Co2Emission(1337.0)),
        );

        assert_eq!(Co2Emission(1337.0), state.accumulated_co2_emission);
    }

    #[test]
    fn test_playing_cards_should_not_result_in_negative_emissions() {
        let initial_state = GameState::default();
        let card = Card {
            title: String::from("Bribe authorities"),
            help_text: String::from("A blind eye is turned for your increasing emissions..."),
            delta_profit: Money(0.0),
            delta_co2: Co2Emission(-1.0),
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(card));

        assert_eq!(Co2Emission(0.0), state.accumulated_co2_emission,);
    }

    #[test]
    fn test_playing_cards_should_preserve_history() {
        let initial_state = GameState::default();
        let first_card = Card {
            title: String::from("Bribe authorities"),
            help_text: String::from("A blind eye is turned for your increasing emissions..."),
            delta_profit: Money(-5.0),
            delta_co2: Co2Emission(5.0),
        };
        let second_card = Card {
            title: String::from("Win machinery"),
            help_text: String::from("Congrats on your new solar-battery powered washing machine!"),
            delta_profit: Money(3.0),
            delta_co2: Co2Emission(-2.0),
        };

        let intermittent_state = game_state_reducer(initial_state, Action::PlayCard(first_card));
        let state = game_state_reducer(intermittent_state, Action::PlayCard(second_card));

        assert_eq!(
            state.played_cards.first().unwrap().title,
            String::from("Bribe authorities")
        );
        assert_eq!(
            state.played_cards.last().unwrap().title,
            String::from("Win machinery")
        );
    }

    #[test]
    fn test_reaching_negative_profit_results_in_game_over() {
        let mut initial_state = GameState::default();
        initial_state.finance.capital = BANKRUPTCY_THRESHOLD;
        let played_card_meta = Card {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: Money(-1.0) - initial_state.finance.capital,
            delta_co2: Co2Emission(0.0),
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(PlaythroughStatus::GameOver, state.playthrough_status);
    }

    #[test]
    fn test_reaching_zero_profit_does_not_result_in_game_over() {
        let mut initial_state = GameState::default();
        initial_state.finance.capital = BANKRUPTCY_THRESHOLD;
        let played_card_meta = Card {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: Money(0.0) - initial_state.finance.capital,
            delta_co2: Co2Emission(0.0),
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(PlaythroughStatus::Ongoing, state.playthrough_status);
    }

    #[test]
    fn test_reaching_higher_co2_than_the_threshold_results_in_game_over() {
        let initial_state = GameState::default();
        let played_card_meta = Card {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: Money(-5.0),
            delta_co2: CATASTROPHIC_POLLUTION_THRESHOLD + Co2Emission(1.0),
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(PlaythroughStatus::GameOver, state.playthrough_status);
    }

    #[test]
    fn test_completing_the_required_rounds_results_in_winning() {
        let mut state = GameState::default();
        let played_card_meta = Card {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: Money(100.0),
            delta_co2: Co2Emission(0.0),
        };

        for _ in 0..ROUNDS_TO_BEAT_THE_GAME + 1 {
            state = game_state_reducer(state, Action::PlayCard(played_card_meta.clone()));
        }

        assert_eq!(PlaythroughStatus::Beaten, state.playthrough_status);
    }

    #[test]
    fn test_failure_to_reach_profit_target_at_round_end_results_game_over() {
        let state = GameState {
            finance: Finance {
                capital: Money(0.0),
                expenses: Money(1.0),
            },
            accumulated_co2_emission: Co2Emission(0.0),
            played_cards: vec![],
            hand: Hand::default(),
            deck: Deck::default(),
            playthrough_status: PlaythroughStatus::Ongoing,
        };
        let card = Card {
            title: String::new(),
            help_text: String::new(),
            delta_profit: Money(0.0),
            delta_co2: Co2Emission(0.0),
        };

        let final_state = game_state_reducer(state, Action::PlayCard(card));

        assert_eq!(PlaythroughStatus::GameOver, final_state.playthrough_status)
    }
}
