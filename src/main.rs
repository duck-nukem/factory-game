use state::{GameState, PlaythroughStatus};
use tui::event_loop;

pub mod card;
mod finance;
mod math;
pub mod state;
pub mod tui;

fn main() {
    let mut state = GameState::default();

    loop {
        state = event_loop(state);

        if state.playthrough_status == PlaythroughStatus::GameOver {
            println!("{state}\nGame over! Thanks for playing");
            break;
        }

        if state.playthrough_status == PlaythroughStatus::Beaten {
            println!(
                "{state}\nYou win, congratulations! The game will continue, but you can exit any time now"
            );
        }
    }
}
