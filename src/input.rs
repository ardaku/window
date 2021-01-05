// Window
// Copyright © 2019-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

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
