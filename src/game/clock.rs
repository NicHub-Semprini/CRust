use std::marker::PhantomData;
use std::time::Instant;

pub struct OpenClock;
pub struct ClosedClock;

pub struct Clock<State = ClosedClock> {
    start_time: Option<Instant>,
    total_seconds: f32,
    last_seconds: f32,
    state: PhantomData<State>,
}

impl Clock<ClosedClock> {
    pub fn open(&self) -> Clock<OpenClock> {
        Clock {
            start_time: Some(Instant::now()),
            total_seconds: self.total_seconds,
            last_seconds: 0.0,
            state: PhantomData::<OpenClock>,
        }
    }
}

impl Clock<OpenClock> {
    pub fn close(&self) -> Clock<ClosedClock> {
        let last_seconds = match self.start_time {
            Some(time) => Instant::now().duration_since(time).as_secs_f32(),
            None => 0.0,
        };
        Clock {
            start_time: None,
            total_seconds: self.total_seconds + last_seconds,
            last_seconds,
            state: PhantomData::<ClosedClock>,
        }
    }
}

impl<State> Clock<State> {
    pub fn get_total_seconds(&self) -> f32 {
        match self.start_time {
            Some(time) => self.total_seconds + Instant::now().duration_since(time).as_secs_f32(),
            None => self.total_seconds,
        }
    }

    pub fn get_last_seconds(&self) -> f32 {
        match self.start_time {
            Some(time) => Instant::now().duration_since(time).as_secs_f32(),
            None => self.last_seconds,
        }
    }
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            start_time: None,
            total_seconds: 0.0,
            last_seconds: 0.0,
            state: PhantomData::default(),
        }
    }
}
