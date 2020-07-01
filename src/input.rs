//! User input from the window.

pub use human::{Input, renumber, rumble, GameInput, Mode, TextInput, UiInput};
use pasts::prelude::*;

/// Get user input from all connected human interface devices.
pub async fn input() -> Input {
    loop {
        match [human::input().fut(), crate::ffi::input().fut()].select().await {
            (0, Input::Text(_)) => { /* ignore terminal text */ },
            (0, Input::Game(id, game)) => break Input::Game(id, game),
            (0, Input::Ui(_)) => { /* ignore terminal mouse events */ },
            (1, Input::Text(text)) => break Input::Text(text),
            (1, Input::Game(_, _)) => { /* never generated, ignore */ },
            (1, Input::Ui(ui)) => break Input::Ui(ui),
            (_, _) => unreachable!(),
        }
    }
}
