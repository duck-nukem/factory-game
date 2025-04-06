use std::fmt::Display;

use crate::card::{CardCollection, CardMeta};

#[derive(Debug, PartialEq, Eq)]
pub enum PlaythroughStatus {
    Ongoing,
    GameOver,
    Beaten,
}

#[derive(Debug)]
pub struct GameState {
    pub accrued_profit: f64,
    pub accumulated_co2_emission: f64,
    pub played_cards: Vec<CardMeta>,
    pub deck: CardCollection,
    pub playthrough_status: PlaythroughStatus,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0}¢ & {1} tCO₂e @ Round {2}",
            self.accrued_profit,
            self.accumulated_co2_emission,
            self.played_cards.len()
        )
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            accrued_profit: 0.0,
            accumulated_co2_emission: 0.0,
            played_cards: vec![],
            deck: CardCollection { cards: vec![] },
            playthrough_status: PlaythroughStatus::Ongoing,
        }
    }
}

#[must_use]
pub const fn initialize_state() -> GameState {
    GameState {
        accrued_profit: 0.0,
        accumulated_co2_emission: 0.0,
        played_cards: vec![],
        deck: CardCollection { cards: vec![] },
        playthrough_status: PlaythroughStatus::Ongoing,
    }
}

pub enum Action {
    UpdateProfit(f64),
    SetProfitExactly(f64),
    UpdateCo2Emission(f64),
    SetCo2Exactly(f64),
    PlayCard(CardMeta),
}

const BANKRUPTCY_THRESHOLD: f64 = 0.0;
const CATASTROPHIC_POLLUTION_THRESHOLD: f64 = 100.0;
const ROUNDS_TO_BEAT_THE_GAME: usize = 8;

#[must_use]
pub fn game_state_reducer(state: GameState, action: Action) -> GameState {
    match action {
        Action::UpdateProfit(incoming_amount) => GameState {
            accrued_profit: state.accrued_profit + incoming_amount,
            ..state
        },
        Action::SetProfitExactly(amount) => GameState {
            accrued_profit: amount,
            ..state
        },
        Action::UpdateCo2Emission(co2_emission) => {
            let accumulated_co2_emission = state.accumulated_co2_emission + co2_emission;

            if accumulated_co2_emission < 0.0 {
                return state;
            }

            GameState {
                accumulated_co2_emission,
                ..state
            }
        }
        Action::SetCo2Exactly(co2_emission) => GameState {
            accumulated_co2_emission: co2_emission,
            ..state
        },
        Action::PlayCard(card) => {
            let accrued_profit = state.accrued_profit + card.delta_profit;
            let mut accumulated_co2_emission = state.accumulated_co2_emission + card.delta_co2;
            let played_cards: Vec<CardMeta> =
                state.played_cards.into_iter().chain(vec![card]).collect();

            if accumulated_co2_emission < 0.0 {
                accumulated_co2_emission = 0.0;
            }

            let is_bankrupt = accrued_profit < BANKRUPTCY_THRESHOLD;
            let is_pollution_catastrophic =
                accumulated_co2_emission >= CATASTROPHIC_POLLUTION_THRESHOLD;
            let has_player_completed_all_required_levels =
                played_cards.len() > ROUNDS_TO_BEAT_THE_GAME;

            let playthrough_status = if is_bankrupt || is_pollution_catastrophic {
                PlaythroughStatus::GameOver
            } else if has_player_completed_all_required_levels {
                PlaythroughStatus::Beaten
            } else {
                state.playthrough_status
            };

            GameState {
                accrued_profit,
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
    use crate::card::CardMeta;

    use super::*;

    #[test]
    fn test_can_acquire_profit() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::UpdateProfit(1.0));

        assert_eq!(1.0, state.accrued_profit);
    }

    #[test]
    fn test_can_acquire_negative_profit() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::UpdateProfit(-1.0));

        assert_eq!(-1.0, state.accrued_profit);
    }

    #[test]
    fn test_can_set_profit_to_any_value() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::SetProfitExactly(1337.0));

        assert_eq!(1337.0, state.accrued_profit);
    }

    #[test]
    fn test_can_increase_co2_emission() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::UpdateCo2Emission(1.0));

        assert_eq!(1.0, state.accumulated_co2_emission);
    }

    #[test]
    fn test_can_reduce_co2_emission() {
        let initial_state = GameState::default();

        let intermittent_state = game_state_reducer(initial_state, Action::SetCo2Exactly(5.0));
        let state = game_state_reducer(intermittent_state, Action::UpdateCo2Emission(-1.0));

        assert_eq!(4.0, state.accumulated_co2_emission);
    }

    #[test]
    fn test_co2_emission_cannot_be_negative() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::UpdateCo2Emission(-1.0));

        assert_eq!(0.0, state.accumulated_co2_emission);
    }

    #[test]
    fn test_can_set_co2_emission_to_any_value() {
        let initial_state = GameState::default();

        let state = game_state_reducer(initial_state, Action::SetCo2Exactly(1337.0));

        assert_eq!(1337.0, state.accumulated_co2_emission);
    }

    #[test]
    fn test_playing_cards_should_not_result_in_negative_emissions() {
        let initial_state = GameState::default();
        let card = CardMeta {
            title: String::from("Bribe authorities"),
            help_text: String::from("A blind eye is turned for your increasing emissions..."),
            delta_profit: 0.0,
            delta_co2: -1.0,
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(card));

        assert_eq!(0.0, state.accumulated_co2_emission,);
    }

    #[test]
    fn test_playing_cards_should_preserve_history() {
        let initial_state = GameState::default();
        let first_card = CardMeta {
            title: String::from("Bribe authorities"),
            help_text: String::from("A blind eye is turned for your increasing emissions..."),
            delta_profit: -5.0,
            delta_co2: 5.0,
        };
        let second_card = CardMeta {
            title: String::from("Win machinery"),
            help_text: String::from("Congrats on your new solar-battery powered washing machine!"),
            delta_profit: 3.0,
            delta_co2: -2.0,
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
        initial_state.accrued_profit = BANKRUPTCY_THRESHOLD;
        let played_card_meta = CardMeta {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: -1.0,
            delta_co2: 0.0,
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(PlaythroughStatus::GameOver, state.playthrough_status);
    }

    #[test]
    fn test_reaching_zero_profit_does_not_result_in_game_over() {
        let mut initial_state = GameState::default();
        initial_state.accrued_profit = BANKRUPTCY_THRESHOLD;
        let played_card_meta = CardMeta {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: 0.0,
            delta_co2: 0.0,
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(PlaythroughStatus::Ongoing, state.playthrough_status);
    }

    #[test]
    fn test_reaching_higher_co2_than_the_threshold_results_in_game_over() {
        let initial_state = GameState::default();
        let played_card_meta = CardMeta {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: -5.0,
            delta_co2: CATASTROPHIC_POLLUTION_THRESHOLD + 1.0,
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(PlaythroughStatus::GameOver, state.playthrough_status);
    }

    #[test]
    fn test_reaching_upper_co2_threshold_results_in_game_over() {
        let initial_state = GameState::default();
        let played_card_meta = CardMeta {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: -5.0,
            delta_co2: CATASTROPHIC_POLLUTION_THRESHOLD,
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(PlaythroughStatus::GameOver, state.playthrough_status);
    }

    #[test]
    fn test_completing_the_required_rounds_results_in_winning() {
        let mut state = GameState::default();
        let played_card_meta = CardMeta {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: 1.0,
            delta_co2: 0.0,
        };

        for _ in 0..ROUNDS_TO_BEAT_THE_GAME + 1 {
            state = game_state_reducer(state, Action::PlayCard(played_card_meta.clone()));
        }

        assert_eq!(PlaythroughStatus::Beaten, state.playthrough_status);
    }
}
