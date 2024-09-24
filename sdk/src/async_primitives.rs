
pub(crate) type JoinHandle<T> = tokio::task::JoinHandle<T>;

pub(crate) type ChannelSender<T> = tokio::sync::mpsc::Sender<T>;
pub(crate) type ChannelReceiver<T> = tokio::sync::mpsc::Receiver<T>;

pub(crate) fn unbounded_channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    tokio::sync::mpsc::unbounded_channel()
}