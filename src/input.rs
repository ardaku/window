//! User input from the window.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use human::Input;

struct InputListener<T>
where
    T: Future<Output = Input>,
{
    original: T,
    additional: crate::ffi::InputListener,
}

/// Get an input listener that gets additional input reported by the window.
pub fn input() -> impl Future<Output = Input> + Unpin {
    let original = Input::listener();
    let additional = crate::ffi::InputListener;

    InputListener {
        original,
        additional,
    }
}

impl<T> Future for InputListener<T>
where
    T: Future<Output = Input> + Unpin,
{
    type Output = Input;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Input> {
        let original = Pin::new(&mut self.original);

        if let Poll::Ready(input) = original.poll(cx) {
            Poll::Ready(input)
        } else {
            let additional = Pin::new(&mut self.additional);
            additional.poll(cx)
        }
    }
}
