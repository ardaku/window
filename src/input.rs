//! User input from the window.

pub use human::{
    renumber, rumble, set_mode, GameInput, Input, Mode, TextInput, UiInput,
};
use pasts::prelude::*;

/// Get user input from all connected human interface devices.
pub async fn input() -> Input {
    [human::input().fut(), crate::ffi::input().fut()]
        .select()
        .await
        .1
}
