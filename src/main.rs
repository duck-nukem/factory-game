use state::{PlaythroughStatus, initialize_state};
use tui::event_loop;

pub mod card;
pub mod state;
pub mod tui;

fn main() {
    let mut state = initialize_state();

    loop {
        state = event_loop(state);

        if state.playthrough_status == PlaythroughStatus::GameOver {
            println!("{state}\nGame over! Thanks for playing");
            break;
        }

        if state.playthrough_status == PlaythroughStatus::Beaten {
            println!("{state}\nYou win, congratulations!");
            break;
        }
    }
}
