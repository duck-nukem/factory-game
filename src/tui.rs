use std::io;

use crate::state::{Action, GameState, PlaythroughStatus, game_state_reducer};

#[must_use]
pub fn ask(question: &str) -> String {
    let mut input = String::new();
    println!("{question} ");
    io::stdin().read_line(&mut input).unwrap_or(0);

    input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
}

pub fn play_game(state: GameState) -> Option<GameState> {
    println!("============");
    match state.playthrough_status {
        PlaythroughStatus::Ongoing => {
            println!("{state}");
        }
        PlaythroughStatus::Beaten => {
            println!("YOU WON! The game goes on!");
            println!("{state}");
        }
        PlaythroughStatus::GameOver => {
            println!("Game over, you made it to Round {0}", state.get_round());
            return None;
        }
    }

    let round = game_state_reducer(state, Action::DrawCards(3));
    println!("{0}", round.hand);

    let chosen_card = ask("Pick one")
        .parse()
        .ok()
        .and_then(|i| round.hand.pick_card(i));
    let next_round = if let Some(chosen_card) = chosen_card {
        println!("Selected: {chosen_card}");
        let action = Action::PlayCard(chosen_card.to_owned());
        game_state_reducer(round, action)
    } else {
        println!("Invalid selection, try again");
        round
    };
    play_game(next_round)
}
