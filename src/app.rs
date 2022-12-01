use ordered_float::*;
use std::{
    error::Error,
    fs,
    path::Path,
    time::{Duration, Instant},
};
use tui::{
    style::{Color, Modifier, Style},
    widgets::TableState,
};
use super::cube::gen_scramble;

pub enum Screen {
    Default,
    Help,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ActiveBlock {
    Tools,
    Help,
    Timer,
    Times,
    Scramble,
    Stats,
    Home,
    Main,
}

pub struct Route {
    pub selected_block: ActiveBlock,
    pub active_block: ActiveBlock,
}

impl Route {
    fn default() -> Self {
        Self {
            selected_block: ActiveBlock::Times,
            active_block: ActiveBlock::Home,
        }
    }

    pub fn esc(&mut self) {
        if self.active_block != ActiveBlock::Home {
            self.active_block = ActiveBlock::Home;
        }
    }

    pub fn enter(&mut self) {
        self.active_block = self.selected_block;
    }
}

#[derive(Clone, Copy)]
pub struct Time {
    pub time: f32,
    pub ao5: Option<OrderedFloat<f32>>,
    pub ao12: Option<OrderedFloat<f32>>,
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

        self.ao5 = if tr.len() >= 5 {
            let set = &tr[0..5];
            Some(Times::calc_aon(set))
        } else {
            None
        };
        self.ao12 = if tr.len() >= 12 {
            let set = &tr[0..12];
            Some(Times::calc_aon(set))
        } else {
            None
        };
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&self.time.to_string())?;
        Ok(())
    }
}

pub struct Times {
    pub times: Vec<Time>,
    pub pbsingle: Option<OrderedFloat<f32>>,
    pub pbao5: Option<OrderedFloat<f32>>,
    pub pbao12: Option<OrderedFloat<f32>>,
    pub ao100: Option<OrderedFloat<f32>>,
    pub ao1k: Option<OrderedFloat<f32>>,
    pub rollingavg: Option<OrderedFloat<f32>>,
    pub sum: OrderedFloat<f32>,
}

impl Times {
    pub fn new() -> Self {
        Self {
            times: vec![],
            pbsingle: None,
            pbao5: None,
            pbao12: None,
            ao100: None,
            ao1k: None,
            rollingavg: None,
            sum: OrderedFloat(0.0),
        }
    }

    pub fn insert(&mut self, time: Time) {
        self.times.push(time);
        Times::update_best(&mut self.pbsingle, Some(OrderedFloat(time.time)));
        Times::update_best(&mut self.pbao5, time.ao5);
        Times::update_best(&mut self.pbao12, time.ao12);

        if self.times.len() >= 100 {
            let mut tr = self.times.clone();
            tr.reverse();
            self.ao100 = Some(Times::calc_aon(&tr[0..100]));
            if self.times.len() >= 1000 {
                self.ao1k = Some(Times::calc_aon(&tr[0..1000]));
            }
        }

        self.sum += OrderedFloat(time.time);

        self.rollingavg = match self.rollingavg {
            Some(_) => Some(self.sum / self.times.len() as f32),
            None => Some(OrderedFloat(time.time)),
        }
    }

    fn update_best(curr: &mut Option<OrderedFloat<f32>>, t: Option<OrderedFloat<f32>>) {
        let new = match t {
            Some(x) => x,
            None => return,
        };

        match curr {
            Some(v) => {
                if new < *v {
                    *curr = Some(OrderedFloat(*new));
                }
            }
            None => *curr = Some(OrderedFloat(*new)),
        }
    }

    fn calc_aon(set: &[Time]) -> OrderedFloat<f32> {
        let mut t = set
            .iter()
            .take(set.len())
            .map(|v| OrderedFloat(v.time))
            .collect::<Vec<OrderedFloat<f32>>>();
        // Remove best and worst time
        t.sort();
        t.pop();
        t.remove(0);

        let mut sum = OrderedFloat(0.0);
        let _ = t.iter().map(|v| sum += v).collect::<Vec<()>>();
        sum / OrderedFloat(t.len() as f32)
    }
}

#[derive(Debug)]
pub struct CubeTimer {
    pub starttime: Option<Instant>,
    pub on: bool,
    pub lasttime: Option<Duration>,
}

impl CubeTimer {
    pub fn default() -> Self {
        Self {
            starttime: None,
            on: false,
            lasttime: None,
        }
    }

    pub fn space_press(&mut self) -> Option<Time> {
        match self.on {
            false => {
                self.timer_on();
                None
            }
            true => Some(self.timer_off()),
        }
    }

    fn timer_on(&mut self) {
        self.on = true;
        self.starttime = Some(Instant::now());
    }

    fn timer_off(&mut self) -> Time {
        self.on = false;
        self.lasttime = Some(self.elapsed());
        self.starttime = None;
        Time::from(
            self.lasttime
                .unwrap_or(Duration::from_secs(0))
                .as_secs_f32(),
        )
    }

    fn elapsed(&self) -> Duration {
        match self.starttime {
            Some(v) => v.elapsed(),
            None => Duration::new(0, 0),
        }
    }

    pub fn text(&self) -> String {
        match self.starttime {
            Some(v) => format!("{:.1}", v.elapsed().as_secs_f32()),
            None => format!(
                "{:.3}",
                self.lasttime
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs_f32()
            ),
        }
    }
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub enum Tool {
    Welcome,
    Chart,
}

pub struct App<'a> {
    pub tick_rate: Duration,
    pub timer: CubeTimer,
    pub route: Route,
    pub path: &'a Path,
    pub pos: (usize, usize),
    pub times: Times,
    pub times_state: TableState,
    layout: Vec<Vec<ActiveBlock>>,
    pub scramble: String,
    pub active_screen: Screen,
    pub tool: Tool,
}

impl<'a> App<'a> {
    pub fn new(tick_rate: Duration, path: &'a Path) -> Result<Self, Box<dyn Error>> {
        // Construct app
        Ok(App {
            tick_rate,
            timer: CubeTimer::default(),
            route: Route::default(),
            path,
            times: Times::new(),
            times_state: TableState::default(),
            pos: (0, 2),
            layout: vec![
                vec![ActiveBlock::Tools, ActiveBlock::Timer, ActiveBlock::Times],
                vec![ActiveBlock::Scramble, ActiveBlock::Stats, ActiveBlock::Main],
            ],
            scramble: gen_scramble(),
            active_screen: Screen::Default,
            tool: Tool::Welcome,
        })
    }

    pub fn load_times(&mut self) -> Result<(), Box<dyn Error>> {
        // Do file stuff
        // fs::create_dir_all(self.path)?;

        // Create file if it doesn't exist
        match fs::File::open(&self.path) {
            Err(_) => _ = fs::File::create(&self.path)?,
            Ok(_) => (),
        };

        let mut times: Vec<Time> = fs::read_to_string(&self.path)?
            .lines()
            .filter_map(|v| v.parse::<f32>().ok())
            .map(|v| Time::from(v))
            .collect();

        for time in &mut times {
            time.gen_stats(&self.times.times);
            self.times.insert(*time);
        }
        Ok(())
    }

    pub fn write_times(&self) -> Result<(), Box<dyn Error>> {
        let write_data: Vec<u8> = self
            .times
            .times
            .iter()
            .flat_map(|v| format!("{}\n", v.to_string()).bytes().collect::<Vec<u8>>())
            .collect();
        fs::write(&self.path, write_data)?;
        Ok(())
    }

    pub fn esc(&mut self) {
        match self.active_screen {
            Screen::Default => self.route.esc(),
            Screen::Help => self.active_screen = Screen::Default,
        }
    }

    pub fn help(&mut self) {
        self.active_screen = Screen::Help;
    }

    pub fn get_border_style_from_id(&self, id: ActiveBlock) -> Style {
        let style = Style::default();

        if id == self.route.active_block {
            return style.fg(Color::LightGreen).add_modifier(Modifier::BOLD);
        } else if id == self.route.selected_block {
            return style.fg(Color::LightBlue).add_modifier(Modifier::BOLD);
        } else {
            return style.fg(Color::Gray);
        }
    }

    pub fn get_highlight_style_from_id(&self, id: ActiveBlock) -> Style {
        let style = Style::default().add_modifier(Modifier::BOLD);

        if id == self.route.active_block {
            return style.fg(Color::LightGreen);
        } else if id == self.route.selected_block {
            return style.fg(Color::LightBlue);
        } else {
            return style.fg(Color::White);
        }
    }

    pub fn del(&mut self) {
        match self.route.active_block {
            ActiveBlock::Times => self.del_time(),
            _ => (),
        }
    }

    pub fn mv(&mut self, dir: Dir) {
        match self.route.active_block {
            ActiveBlock::Home => {
                match dir {
                    Dir::Up => self.mv_up(),
                    Dir::Down => self.mv_down(),
                    Dir::Right => self.mv_right(),
                    Dir::Left => self.mv_left(),
                }
                self.route.selected_block = self.layout[self.pos.0][self.pos.1];
            }
            ActiveBlock::Times => match dir {
                Dir::Up => self.previous_time(),
                Dir::Down => self.next_time(),
                _ => (),
            },
            _ => (),
        }
    }

    fn mv_up(&mut self) {
        if (self.pos.1) as i32 - 1 >= 0 {
            self.pos.1 -= 1;
        }
    }

    fn mv_down(&mut self) {
        if self.pos.1 + 1 < self.layout[self.pos.0].len() {
            self.pos.1 += 1;
        }
    }

    fn mv_right(&mut self) {
        if self.layout.len() > self.pos.0 + 1 {
            let max = self.layout[self.pos.0 + 1].len() - 1;
            if self.pos.1 + 1 > max {
                self.pos.1 = max;
            }
            self.pos.0 += 1;
        }
    }

    pub fn mv_left(&mut self) {
        if (self.pos.0) as i32 - 1 >= 0 {
            self.pos.0 -= 1;
        }
    }

    pub fn next_time(&mut self) {
        let len = self.times.times.len();
        if len == 0 {
            return;
        }
        let i = match self.times_state.selected() {
            Some(i) => {
                if i >= self.times.times.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.times_state.select(Some(i));
    }

    fn previous_time(&mut self) {
        let len = self.times.times.len();
        if len == 0 {
            return;
        }
        let i = match self.times_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.times.times.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.times_state.select(Some(i));
    }

    fn del_time(&mut self) {
        match self.times_state.selected() {
            Some(v) => {
                // Edge cases (literally)
                let len = self.times.times.len();
                if len <= 0 || v >= len {
                    return;
                }
                self.times.times.remove(len - v - 1);
                // Go up one if selection fell off
                if v == self.times.times.len() {
                    self.previous_time();
                }
            },
            None => (),
        };
    }

    pub fn new_scramble(&mut self) {
        self.scramble = gen_scramble();
    }

    pub fn on_tick(&self) {
        ()
    }
}
