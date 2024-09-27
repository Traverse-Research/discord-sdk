pub type JoinHandle<T> = std::thread::JoinHandle<T>;

pub type Sender<T> = crossbeam_channel::Sender<T>;
pub type SendError<T> = crossbeam_channel::SendError<T>;
pub type Receiver<T> = crossbeam_channel::Receiver<T>;
pub type RecvError = crossbeam_channel::RecvError;
pub struct Interval {
    init_time: std::time::Instant,
    last_tick: Option<std::time::Instant>,
    period: std::time::Duration,
}

impl Interval {
    pub fn tick(&mut self) {
        let Some(last_tick) = &mut self.last_tick else {
            self.last_tick = Some(self.init_time);
            return;
        };

        let now = std::time::Instant::now();

        let elapsed_since_tick = now.duration_since(*last_tick);
        loop {
            if elapsed_since_tick > self.period {
                // Enough time has elapsed, between ticks so we don't need to yield.
                // Before that though, increment last_tick with the period.
                // This mimics the behaviour of tokio::time::Interval's 
                // MissedTickBehaviour::Burst.
                // TODO: If other MissedTickBehaviours are needed, they should be implemented.
                *last_tick += self.period;
                return;
            } else {
                // Not enough time has passed between ticks, so we must yield and retry once it's this thread's turn again.
                std::thread::yield_now();
            }
        }
    }
}

pub mod time {
    pub fn interval(period: std::time::Duration) -> super::Interval {
        super::Interval {
            init_time: std::time::Instant::now(),
            last_tick: None,
            period,
        }
    }
}

pub(crate) fn unbounded_channel<T>() -> (Sender<T>, Receiver<T>) {
    crossbeam_channel::unbounded()
}

pub(crate) fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    crossbeam_channel::bounded(buffer)
}


// pub(crate) trait AsyncAgnosticReceiver<T> {
//     #[sdk_async]
//     fn recv_async(&self) -> Option<T>;
// }

// impl<T> AsyncAgnosticReceiver<T> for Receiver<T> {
//     #[sdk_async]
//     fn recv_async(&self) -> Option<T> {
//         self.recv().ok()
//     }
// }
