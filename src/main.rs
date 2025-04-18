use state::GameState;
use tui::play_game;

mod card;
mod emission;
mod finance;
mod gui;
mod math;
mod state;
mod tui;

fn main() {
    play_game(GameState::default());
}
