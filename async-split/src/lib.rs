#[cfg(feature = "async")]
pub mod async_types;
#[cfg(feature = "async")]
use async_types as types;
#[cfg(feature = "async")]
pub mod async_impls;

#[cfg(feature = "async")]
#[macro_export]
macro_rules! async_await {
    ($fut:expr) => {
        (($fut).await)
    };
}

#[cfg(not(feature = "async"))]
pub mod non_async_types;
#[cfg(not(feature = "async"))]
use non_async_types as types;
#[cfg(not(feature = "async"))]
pub mod non_async_impls;

#[cfg(not(feature = "async"))]
#[macro_export]
macro_rules! async_await {
    ($fut:expr) => {
        $fut
    };
}

pub use types::*;

pub struct ChannelReceiver<T> {
    receiver: Receiver<T>,
}

pub struct ChannelSender<T> {
    sender: Sender<T>,
}
pub mod channel {
    use crate::{types, ChannelReceiver, ChannelSender};

    fn new<T>(buffer: Option<usize>) -> (ChannelSender<T>, ChannelReceiver<T>) {
        let (sender, receiver) = if let Some(buffer) = buffer {
            types::channel(buffer)
        } else {
            types::unbounded_channel()
        };
        (ChannelSender { sender }, ChannelReceiver { receiver })
    }

    pub fn unbounded<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
        new(None)
    }

    pub fn bounded<T>(buffer: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
        new(Some(buffer))
    }
}
