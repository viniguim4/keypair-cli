use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Clock {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub milliseconds: u32,
}

impl Clock{
    pub fn current_time() -> Self {
        // 1. Get the current time since the UNIX epoch.
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!");

        // 2. Get the milliseconds separately.
        let milliseconds = current_time.subsec_millis();

        // 3. Calculate the number of seconds and separate them into hours, minutes, and seconds.
        let seconds = current_time.as_secs();
        let hours = (seconds / 3600) % 24; 
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;

        // 4. Display the current time in the desired format.
        Clock { hours, minutes, seconds, milliseconds }
    }

    fn sec2mil(&self) -> u64 {
        self.seconds * 1000
    }

    fn total(&self) -> u64 {
        self.sec2mil() + self.milliseconds as u64
    }
}

pub fn latency(clock1: &Clock, clock2: &Clock) -> u64 {
    let latency = clock2.total() - clock1.total();
    latency
}