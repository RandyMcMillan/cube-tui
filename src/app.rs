use super::timer::*;
use std::time::{Duration, Instant};
use tui::{style::Color, widgets::TableState};

#[derive(PartialEq, Eq)]
pub enum RouteId {
    Timer,
    Times,
    Scramble,
    Main,
    Home,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

const SELECTABLE: [[ActiveBlock; 2]; 2] = [
    [ActiveBlock::Timer, ActiveBlock::Times],
    [ActiveBlock::Scramble, ActiveBlock::Main],
];

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

    pub fn esc(&mut self) {
        if self.active_block != ActiveBlock::Home {
            self.active_block = ActiveBlock::Home;
        }
    }

    pub fn set_pos(&mut self, (x, y): (usize, usize)) {
        self.selected_block = SELECTABLE[x][y];
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
        Self {
            time,
            ao5: None,
            ao12: None,
        }
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
    pub pos: (usize, usize),
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
            pos: (0, 1),
        }
    }

    pub fn get_color_from_id(&self, id: ActiveBlock) -> Color {
        let mut color = Color::Gray;
        if id == self.route.selected_block {
            color = Color::LightBlue;
        }
        if id == self.route.active_block {
            color = Color::LightGreen;
        } 
        color
    }

    pub fn mv_horiz(&mut self, right: bool) {
        if right && (self.pos.0 + 1 < SELECTABLE[self.pos.1].len()) {
            self.pos.0 += 1;
        } else if !right && ((self.pos.0) as i32 - 1 >= 0) {
            self.pos.0 -= 1;
        }

        self.route.set_pos(self.pos);
    }

    pub fn mv_vert(&mut self, up: bool) {
        if up && ((self.pos.1) as i32 - 1 >= 0) {
            self.pos.1 -= 1;
        } else if !up && (self.pos.1 + 1 < SELECTABLE[self.pos.0].len()) {
            self.pos.1 += 1;
        }

        self.route.set_pos(self.pos);
    }

    pub fn on_tick(&self) {
        ()
    }
}
