use float_duration::FloatDuration;
use std::sync::Arc;
use serde_derive::{Serialize, Deserialize};
use std::collections::BTreeMap;

pub type Res<T> = Result<T, Box<::std::error::Error>>;
pub type ClientId = u64;

#[derive(Clone, Serialize, Deserialize)]
pub enum ShowTime {
    Elapsed,
    Remaining,
    None
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TimePresence {
    pub details: String,
    pub state: String,
    pub tooltip: String,

    pub small_image: String,
    pub large_image: String,

    pub show_time: ShowTime,
    pub length: Option<FloatDuration>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Presence {
    pub application_id: u64,
    pub name: String,
    pub elapsed: FloatDuration,
    pub active_presence: usize,
    pub time_presences: Vec<TimePresence>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub current: usize,
    pub presences: Vec<Presence>,

    pub update: bool
}

#[derive(Clone)]
pub enum WinState {
    Opening,
    Tray,
    Open(::sciter::Element),
    Closing
}

pub type MState = ::std::sync::Arc<::std::sync::Mutex<State>>;
pub type WState = ::std::sync::Arc<::std::sync::Mutex<WinState>>;

pub trait Pushi<T> { fn push(&mut self, x: T); }
pub trait Swappi<I> { fn swap(&mut self, i1: I, i2: I); }
pub trait Vecci<T> { fn to_vec(self) -> Vec<T>; }

impl<T> Pushi<T> for BTreeMap<usize, T> {
    fn push(&mut self, x: T) {
        let k = match self.keys().last() { Some(x) => x+1, None => 0 };
        self.insert(k as usize, x);
    }
}

impl<I: ::std::cmp::Ord, T> Swappi<I> for BTreeMap<I, T> {
    fn swap(&mut self, i1: I, i2: I) {
        use std::mem;

        let x = self.remove(&i1).unwrap();
        let y = mem::replace(self.get_mut(&i2).unwrap(), x);
        self.insert(i1, y);
    }
}

impl<T> Vecci<T> for Option<T> {
    fn to_vec(self) -> Vec<T> {
        match self {
            Some(x) => vec![x],
            None => Vec::new()
        }
    }
}

impl State {
    pub fn new () -> State {
        State { current: 0, presences: Vec::new(), update: true }
    }
}

impl TimePresence {
    pub fn new() -> Self {
        TimePresence {
            state: "Hello world!".to_owned(),
            details: "I like dogs!".to_owned(),
            tooltip: "...I hate cats".to_owned(),
            small_image: "yawny".to_owned(),
            large_image: "wuht".to_owned(),
            show_time: ShowTime::Elapsed,
            length: None
        }
    }
}

impl Presence {
    pub fn current(&self) -> Option<&TimePresence> {
        self.time_presences.get(self.active_presence).or_else(|| self.time_presences.first())
    }

    const DEFAULT_APP_ID: u64 = 465995275563958272;

    pub fn new(state: &State) -> Self {
        let name = format!("Presence {}", state.presences.len()+1);
        Presence {name, application_id: Self::DEFAULT_APP_ID, elapsed: FloatDuration::zero(), active_presence: 0, time_presences: Vec::new()}
    }
}