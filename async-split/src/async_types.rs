pub(crate) type JoinHandle<T> = tokio::task::JoinHandle<T>;

pub(crate) type ChannelSender<T> = tokio::sync::mpsc::Sender<T>;
pub(crate) type ChannelReceiver<T> = tokio::sync::mpsc::Receiver<T>;

pub(crate) type Interval = tokio::time::Interval;

pub(crate) fn unbounded_channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    tokio::sync::mpsc::unbounded_channel()
}

pub(crate) fn channel<T>(buffer: usize) -> (ChannelSender<T>, ChannelReceiver<T>) {
    tokio::sync::mpsc::channel(bounds)
}

pub(crate) fn interval(period: std::time::Duration) -> Interval {
    tokio::time::interval(period)
}

pub(crate) trait AsyncAgnosticReceiver<T> {
    #[sdk_async]
    fn recv_async(&self) -> Option<T>;
}

impl<T> AsyncAgnosticReceiver<T> for ChannelReceiver<T> {
    #[sdk_async]
    fn recv_async(&self) -> Option<T> {
        self.recv().await
    }
}
