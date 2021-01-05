// Window
// Copyright © 2019-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Waker};

use human::Input;

// True for async thread, false for main thread.
static PIPE_LOCK: AtomicBool = AtomicBool::new(true);
// Pipe data.
static mut PIPE: (Vec<Input>, Option<Waker>) = (vec![], None);

pub(super) struct InputListener;

impl Future for InputListener {
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
pub(super) unsafe fn push_inputs(inputs: Vec<Input>) {
    if !PIPE_LOCK.load(Ordering::SeqCst) {
        if let Some(waker) = PIPE.1.take() {
            PIPE.0.extend(inputs);
            PIPE_LOCK.store(true, Ordering::SeqCst);
            waker.wake();
        }
    }
}
