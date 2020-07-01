//! User input from the window.

pub use human::{Input, renumber, rumble, GameInput, Mode, TextInput, UiInput, set_mode};
use pasts::prelude::*;

/// Get user input from all connected human interface devices.
pub async fn input() -> Input {
    [human::input().fut(), crate::ffi::input().fut()].select().await.1
}
