use std::io;

use crate::state::{Action, GameState, game_state_reducer};

#[must_use]
pub fn ask(question: &str) -> String {
    let mut input = String::new();
    println!("{question} ");
    io::stdin().read_line(&mut input).unwrap_or(0);

    input
}

#[must_use]
pub fn event_loop(state: GameState) -> GameState {
    clear_screen();
    println!("{state}");
    let round = game_state_reducer(state, Action::DrawCards(3));

    for (index, card) in round.hand.iter().enumerate() {
        println!("{index} -> {card}");
    }

    let mut chosen_card = ask("Pick one");
    chosen_card.retain(|c| !c.is_ascii_whitespace());
    let card_index: usize = chosen_card.parse().unwrap_or_default();
    let card_meta = round.hand.get(card_index);

    match card_meta {
        Some(card) => {
            println!("Selected: {card}");
            let action = Action::PlayCard(card.to_owned());

            game_state_reducer(round, action)
        }
        None => round,
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
