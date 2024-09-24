
pub(crate) type JoinHandle<T> = std::thread::JoinHandle<T>;

pub(crate) type ChannelSender<T> = crossbeam_channel::Sender<T>;
pub(crate) type ChannelReceiver<T> = crossbeam_channel::Receiver<T>;

pub(crate) fn unbounded_channel<T>() -> (ChannelSender<T>, ChannelReceiver<T>) {
    crossbeam_channel::unbounded()
}