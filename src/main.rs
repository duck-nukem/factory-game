use state::GameState;
use tui::play_game;

pub mod card;
mod emission;
mod finance;
mod math;
pub mod state;
pub mod tui;

fn main() {
    play_game(GameState::default());
}
