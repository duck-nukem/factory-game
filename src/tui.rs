use std::io;

use crate::{
    card::{DEFAULT_CARD_META, load_cards},
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
    let cards = load_cards();

    for (index, card) in cards.iter().enumerate() {
        println!("{index} -> {card}");
    }

    let mut chosen_card = ask("Pick one");
    chosen_card.retain(|c| !c.is_ascii_whitespace());
    let card_index: usize = chosen_card.parse().unwrap_or_default();
    let card_meta = cards.get(card_index).unwrap_or(DEFAULT_CARD_META).clone();
    println!("Selected: {card_meta}");
    let action = Action::PlayCard(card_meta);

    game_state_reducer(state, action)
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
