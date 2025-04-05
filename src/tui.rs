use std::io;

use crate::{
    card::{Deck, load_cards},
    state::{Action, GameState, game_state_reducer},
};

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
    let mut cards = load_cards();
    let hand = cards.draw_cards(3);

    for (index, card) in hand.iter().enumerate() {
        println!("{index} -> {card}");
    }

    let mut chosen_card = ask("Pick one");
    chosen_card.retain(|c| !c.is_ascii_whitespace());
    let card_index: usize = chosen_card.parse().unwrap_or_default();
    let card_meta = hand.get(card_index);

    match card_meta {
        Some(card) => {
            println!("Selected: {card}");
            let action = Action::PlayCard(card.clone());

            game_state_reducer(state, action)
        }
        None => state,
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
