use std::io;

use crate::state::{Action, GameState, PlaythroughStatus, game_state_reducer};

#[must_use]
pub fn ask(question: &str) -> String {
    let mut input = String::new();
    println!("{question} ");
    io::stdin().read_line(&mut input).unwrap_or(0);

    input
}

pub fn play_game(state: GameState) -> Option<GameState> {
    clear_screen();

    match state.playthrough_status {
        PlaythroughStatus::Ongoing => {
            println!("{state}");
        }
        PlaythroughStatus::Beaten => {
            println!("YOU WON! The game goes on!");
            println!("{state}");
        }
        PlaythroughStatus::GameOver => {
            println!(
                "Game over, you made it to Round {0}",
                state.played_cards.len()
            );
            return None;
        }
    }

    let round = game_state_reducer(state, Action::DrawCards(3));

    for (index, card) in round.hand.iter().enumerate() {
        println!("{index} -> {card}");
    }

    let mut chosen_card = ask("Pick one");
    chosen_card.retain(|c| !c.is_ascii_whitespace());
    let card_index: usize = chosen_card.parse().unwrap_or_default();

    match round.hand.get(card_index) {
        Some(card) => {
            println!("Selected: {card}");
            let action = Action::PlayCard(card.to_owned());
            play_game(game_state_reducer(round, action))
        }
        None => Some(round),
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
