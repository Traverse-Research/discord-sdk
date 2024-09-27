impl<T> ChannelReceiver<T> {
    pub fn recv(&self) -> Result<T, types::RecvError> {
        todo!();
    }
}

impl<T> ChannelSender<T> {
    pub fn send(&self, msg: T) -> Result<(), types::SendError<T>>  {
        todo!();
    }
}