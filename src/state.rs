use crate::card::CardMeta;

struct GameState {
    accrued_profit: f64,
    accumulated_co2_emission: f64,
    played_cards: Vec<CardMeta>,
}

fn initialize_state() -> GameState {
    return GameState {
        accrued_profit: 0.0,
        accumulated_co2_emission: 0.0,
        played_cards: vec![],
    };
}

enum Action {
    AcquireProfit(f64),
    SetProfitExactly(f64),
    IncreaseCo2Emission(f64),
    ReduceCo2Emission(f64),
    PlayCard(CardMeta),
}

fn game_state_reducer(state: GameState, action: Action) -> GameState {
    let updated_state = match action {
        Action::AcquireProfit(incoming_amount) => GameState {
            accrued_profit: state.accrued_profit + incoming_amount,
            ..state
        },
        Action::SetProfitExactly(amount) => GameState {
            accrued_profit: amount,
            ..state
        },
        Action::IncreaseCo2Emission(co2_emission_increase) => GameState {
            accumulated_co2_emission: state.accumulated_co2_emission + co2_emission_increase,
            ..state
        },
        Action::ReduceCo2Emission(co2_emission_decrease) => GameState {
            accumulated_co2_emission: state.accumulated_co2_emission + co2_emission_decrease,
            ..state
        },
        Action::PlayCard(card) => GameState {
            accrued_profit: state.accrued_profit + card.delta_profit,
            accumulated_co2_emission: state.accumulated_co2_emission + card.delta_co2,
            played_cards: state.played_cards.into_iter().chain(vec![card]).collect(),
            ..state
        },
    };

    return updated_state;
}

#[cfg(test)]
mod tests {
    use crate::card::CardMeta;

    use super::*;

    #[test]
    fn test_can_acquire_profit() {
        let initial_state = initialize_state();

        let state = game_state_reducer(initial_state, Action::AcquireProfit(1.0));

        assert_eq!(1.0, state.accrued_profit);
    }

    #[test]
    fn test_can_acquire_negative_profit() {
        let initial_state = initialize_state();

        let state = game_state_reducer(initial_state, Action::AcquireProfit(-1.0));

        assert_eq!(-1.0, state.accrued_profit);
    }

    #[test]
    fn test_can_set_profit_to_any_value() {
        let initial_state = initialize_state();

        let state = game_state_reducer(initial_state, Action::SetProfitExactly(1337.0));

        assert_eq!(1337.0, state.accrued_profit);
    }

    #[test]
    fn test_can_incur_co2_emission() {
        let initial_state = initialize_state();

        let state = game_state_reducer(initial_state, Action::IncreaseCo2Emission(1.0));

        assert_eq!(1.0, state.accumulated_co2_emission);
    }

    #[test]
    fn test_can_reduce_co2_emission() {
        let initial_state = initialize_state();

        let state = game_state_reducer(initial_state, Action::ReduceCo2Emission(1.0));

        assert_eq!(1.0, state.accumulated_co2_emission);
    }

    #[test]
    fn test_playing_a_card_should_update_profit_and_emission() {
        let initial_state = initialize_state();
        let played_card_meta = CardMeta {
            title: String::from("A card"),
            help_text: String::from("Nobody will read this... will they?"),
            delta_profit: -5.0,
            delta_co2: 5.0,
        };

        let state = game_state_reducer(initial_state, Action::PlayCard(played_card_meta));

        assert_eq!(5.0, state.accumulated_co2_emission);
        assert_eq!(-5.0, state.accrued_profit);
        assert_eq!(
            state.played_cards.first().unwrap().title,
            String::from("A card")
        );
    }

    #[test]
    fn test_playing_cards_should_preserve_history() {
        let initial_state = initialize_state();
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
}
