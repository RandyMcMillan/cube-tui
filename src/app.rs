use super::timer::*;
use std::time::{Duration, Instant};
use tui::widgets::TableState;

pub enum RouteId {
    Tools,
    Help,
    Timer,
    Times,
    Scramble,
    Main,
    Home,
}

pub enum ActiveBlock {
    Tools,
    Help,
    Timer,
    Times,
    Scramble,
    LineGraph,
    BarGraph,
    Stats,
    Home,
    Main,
}

pub struct Route {
    pub id: RouteId,
    pub selected_block: ActiveBlock,
    pub active_block: ActiveBlock,
}

impl Route {
    fn default() -> Self {
        Self {
            id: RouteId::Home,
            selected_block: ActiveBlock::Times,
            active_block: ActiveBlock::Home,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Time {
    pub time: f32,
    pub ao5: Option<f32>,
    pub ao12: Option<f32>,
}

impl Time {
    pub fn from(time: f32) -> Self {
        Self { time, ao5: None, ao12: None }
    }

    pub fn gen_stats(&mut self, times: &Vec<Time>) {
        let mut tr = times.clone();
        tr.push(*self);
        tr.reverse();

        self.ao12 = if tr.len() >= 12 {
            Some(tr.iter().take(12).map(|v| v.time).sum::<f32>() / 12.0)
        } else {
            None
        };
        self.ao5 = if tr.len() >= 5 {
            Some(tr.iter().take(5).map(|v| v.time).sum::<f32>() / 5.0)
        } else {
            None
        };
    }
}

pub struct App {
    pub tick_rate: Duration,
    pub timer: CubeTimer,
    pub route: Route,
    pub times: Vec<Time>,
    pub times_state: TableState,
}

impl App {
    pub fn new(tick_rate: Duration) -> Self {
        App {
            tick_rate,
            timer: CubeTimer::default(),
            route: Route::default(),
            times: vec![],
            times_state: TableState::default(),
        }
    }

    pub fn on_tick(&self) {
        ()
    }
}
