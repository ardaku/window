use human::Input;
use std::{cell::RefCell, future::Future, pin::Pin, task::{Context, Poll, Waker}, sync::atomic::{Ordering, AtomicBool}};

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
