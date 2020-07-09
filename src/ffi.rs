use human::{GameInput, Input, Mode, TextInput, UiInput};
use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll, Waker},
};

// True for async thread, false for main thread.
static PIPE_LOCK: AtomicBool = AtomicBool::new(true);
// Pipe data.
static mut PIPE: (Vec<Input>, Option<Waker>) = (vec![], None);

struct InputFuture;

impl Future for InputFuture {
    type Output = Input;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if PIPE_LOCK.load(Ordering::SeqCst) {
            if let Some(input) = unsafe { PIPE.0.pop() } {
                Poll::Ready(input)
            } else {
                PIPE_LOCK.store(false, Ordering::SeqCst);
                unsafe { PIPE.1 = Some(cx.waker().clone()) };
                Poll::Pending
            }
        } else {
            Poll::Pending
        }
    }
}

// Do not call this function more than once without waiting for some hardware
// event between calls.  Doing so will cause input to be lost.  Only call this
// function from the main thread.
pub(super) unsafe fn push_inputs(inputs: &[Input]) {
    if !PIPE_LOCK.load(Ordering::SeqCst) {
        if let Some(waker) = PIPE.1.take() {
            PIPE.0.extend(inputs);
            PIPE_LOCK.store(true, Ordering::SeqCst);
            waker.wake();
        }
    }
}

pub(super) async fn input() -> Input {
    InputFuture.await
}

pub(super) fn keyboard_back(held: bool) -> Option<Input> {
    if held {
        Some(match human::mode_keyboard() {
            Mode::Ui => Input::Ui(UiInput::Back),
            Mode::Text => Input::Text(TextInput::Back),
            Mode::Game => Input::Game(0, GameInput::Back),
        })
    } else {
        None
    }
}

pub(super) fn key_w(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Up))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::JoyY(-1.0)))
            } else {
                Some(Input::Game(0, GameInput::JoyY(0.0)))
            }
        }
        Mode::Text => None,
    }
}

pub(super) fn key_s(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Down))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::JoyY(1.0)))
            } else {
                Some(Input::Game(0, GameInput::JoyY(0.0)))
            }
        }
        Mode::Text => None,
    }
}

pub(super) fn key_a(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Left))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::JoyX(-1.0)))
            } else {
                Some(Input::Game(0, GameInput::JoyX(0.0)))
            }
        }
        Mode::Text => None,
    }
}

pub(super) fn key_d(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Right))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::JoyX(1.0)))
            } else {
                Some(Input::Game(0, GameInput::JoyX(0.0)))
            }
        }
        Mode::Text => None,
    }
}

fn key_num(held: bool, number: u8) -> Option<Input> {
    if held {
        if let Mode::Game = human::mode_keyboard() {
            return Some(Input::Game(0, GameInput::Slot(number)));
        }
    }
    None
}

pub(super) fn key_one(held: bool) -> Option<Input> {
    key_num(held, 1)
}

pub(super) fn key_two(held: bool) -> Option<Input> {
    key_num(held, 2)
}

pub(super) fn key_three(held: bool) -> Option<Input> {
    key_num(held, 3)
}

pub(super) fn key_four(held: bool) -> Option<Input> {
    key_num(held, 4)
}

pub(super) fn key_five(held: bool) -> Option<Input> {
    key_num(held, 5)
}

pub(super) fn key_six(held: bool) -> Option<Input> {
    key_num(held, 6)
}

pub(super) fn key_seven(held: bool) -> Option<Input> {
    key_num(held, 7)
}

pub(super) fn key_eight(held: bool) -> Option<Input> {
    key_num(held, 8)
}

pub(super) fn key_nine(held: bool) -> Option<Input> {
    key_num(held, 9)
}

pub(super) fn key_ten(held: bool) -> Option<Input> {
    key_num(held, 10)
}

pub(super) fn key_eleven(held: bool) -> Option<Input> {
    key_num(held, 11)
}

pub(super) fn key_twelve(held: bool) -> Option<Input> {
    key_num(held, 12)
}

pub(super) fn key_tab(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Next))
            } else {
                None
            }
        }
        Mode::Game => Some(Input::Game(0, GameInput::JoyPush(held))),
        Mode::Text => Some(Input::Text(TextInput::Tab)),
    }
}

pub(super) fn key_backslash(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Prev))
            } else {
                None
            }
        }
        Mode::Game => Some(Input::Game(0, GameInput::CamPush(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_enter(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Choose))
            } else {
                None
            }
        }
        Mode::Game => Some(Input::Game(0, GameInput::A(held))),
        Mode::Text => Some(Input::Text(TextInput::Enter)),
    }
}

pub(super) fn key_up(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Up))
            } else {
                None
            }
        }
        Mode::Game => Some(Input::Game(0, GameInput::Up(held))),
        Mode::Text => if held {
            Some(Input::Text(TextInput::Up))
        } else {
            None
        },
    }
}

pub(super) fn key_down(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Down))
            } else {
                None
            }
        }
        Mode::Game => Some(Input::Game(0, GameInput::Down(held))),
        Mode::Text => if held {
            Some(Input::Text(TextInput::Down))
        } else {
            None
        },
    }
}

pub(super) fn key_left(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Left))
            } else {
                None
            }
        }
        Mode::Game => Some(Input::Game(0, GameInput::Left(held))),
        Mode::Text => if held {
            Some(Input::Text(TextInput::Left))
        } else {
            None
        },
    }
}

pub(super) fn key_right(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Right))
            } else {
                None
            }
        }
        Mode::Game => Some(Input::Game(0, GameInput::Right(held))),
        Mode::Text => if held {
            Some(Input::Text(TextInput::Right))
        } else {
            None
        },
    }
}

pub(super) fn key_ctrl(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(
            0,
            GameInput::TriggerR(if held { 1.0 } else { 0.0 }),
        )),
        Mode::Text => None,
    }
}

pub(super) fn key_shift(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::B(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_alt(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(
            0,
            GameInput::TriggerL(if held { 1.0 } else { 0.0 }),
        )),
        Mode::Text => None,
    }
}

pub(super) fn key_space(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::V(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_g(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::BumperL(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_f(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::Down(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_r(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::Up(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_i(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::BumperR(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_e(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::BumperR(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_u(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => Some(Input::Game(0, GameInput::BumperL(held))),
        Mode::Text => None,
    }
}

pub(super) fn key_t(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => None,
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::Menu))
            } else {
                None
            }
        }
        Mode::Text => None,
    }
}

pub(super) fn key_k(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Up))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::CamY(-1.0)))
            } else {
                Some(Input::Game(0, GameInput::CamY(0.0)))
            }
        }
        Mode::Text => None,
    }
}

pub(super) fn key_j(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Down))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::CamY(1.0)))
            } else {
                Some(Input::Game(0, GameInput::CamY(0.0)))
            }
        }
        Mode::Text => None,
    }
}

pub(super) fn key_h(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Left))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::CamX(-1.0)))
            } else {
                Some(Input::Game(0, GameInput::CamX(0.0)))
            }
        }
        Mode::Text => None,
    }
}

pub(super) fn key_l(held: bool) -> Option<Input> {
    match human::mode_keyboard() {
        Mode::Ui => {
            if held {
                Some(Input::Ui(UiInput::Right))
            } else {
                None
            }
        }
        Mode::Game => {
            if held {
                Some(Input::Game(0, GameInput::CamX(1.0)))
            } else {
                Some(Input::Game(0, GameInput::CamX(0.0)))
            }
        }
        Mode::Text => None,
    }
}
