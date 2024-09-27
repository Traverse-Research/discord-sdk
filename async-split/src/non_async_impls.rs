use super::non_async_types as types;

impl<T> crate::ChannelReceiver<T> {
    pub fn recv(&self) -> Result<T, types::RecvError> {
        self.receiver.recv()
    }
}

impl<T> crate::ChannelSender<T> {
    pub fn send(&self, msg: T) -> Result<(), types::SendError<T>>  {
        self.sender.send(msg)
    }

    pub fn send_blocking(&self, msg: T) -> Result<(), types::SendError<T>> {
        self.sender.send(msg)
    }
}