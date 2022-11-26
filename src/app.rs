use std::time::{Duration, Instant};
use super::timer::*;

pub enum RouteId {
    Tools,
    Help,
    Time,
    Times,
    Scramble,
    Main,
    Home,
}

pub enum ActiveBlock {
    Tools,
    Help,
    Time,
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

pub struct App {
    pub tick_rate: Duration,
    pub timer: CubeTimer,
    pub route: Route,
}

impl App {
    pub fn new(tick_rate: Duration) -> Self {
        App {
            tick_rate,
            timer: CubeTimer::default(),
            route: Route::default(),
        }
    }

    pub fn on_tick(&self) {
        ()
    }
}
