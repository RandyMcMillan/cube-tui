use ordered_float::*;
use std::{
    env,
    error::Error,
    time::{Duration, Instant},
    fs,
    path::Path,
};
use tui::{
    style::{Color, Modifier, Style},
    widgets::TableState,
};

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

        self.ao12 = if tr.len() >= 12 {
            let mut tmp = tr
                .iter()
                .take(12)
                .map(|v| OrderedFloat(v.time))
                .collect::<Vec<OrderedFloat<f32>>>();
            tmp.sort();
            tmp.pop();
            tmp.remove(0);
            let mut sum = OrderedFloat(0.0);
            let _ = tmp.iter().map(|v| sum += v).collect::<Vec<()>>();
            Some(sum / OrderedFloat(10.0))
        } else {
            None
        };
        self.ao5 = if tr.len() >= 5 {
            let mut tmp = tr
                .iter()
                .take(5)
                .map(|v| OrderedFloat(v.time))
                .collect::<Vec<OrderedFloat<f32>>>();
            tmp.sort();
            tmp.pop();
            tmp.remove(0);
            let mut sum = OrderedFloat(0.0);
            let _ = tmp.iter().map(|v| sum += v).collect::<Vec<()>>();
            Some(sum / OrderedFloat(3.0))
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

#[derive(Debug)]
pub struct CubeTimer {
    starttime: Option<Instant>,
    on: bool,
    lasttime: Duration,
}

impl CubeTimer {
    pub fn default() -> Self {
        Self {
            starttime: None,
            on: false,
            lasttime: Duration::new(0, 0),
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
        self.lasttime = self.elapsed();
        self.starttime = None;
        Time::from(self.lasttime.as_secs_f32())
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
            None => format!("{:.3}", self.lasttime.as_secs_f32()),
        }
    }
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct App<'a> {
    pub tick_rate: Duration,
    pub timer: CubeTimer,
    pub route: Route,
    pub path: &'a Path,
    pub pos: (usize, usize),
    pub times: Vec<Time>,
    pub times_state: TableState,
    layout: Vec<Vec<ActiveBlock>>,
}

impl<'a> App<'a> {
    pub fn new(tick_rate: Duration, path: &'a Path) -> Result<Self, Box<dyn Error>> {
        // Construct app
        Ok(App {
            tick_rate,
            timer: CubeTimer::default(),
            route: Route::default(),
            path,
            times: vec![],
            times_state: TableState::default(),
            pos: (0, 2),
            layout: vec![
                vec![ActiveBlock::Tools, ActiveBlock::Timer, ActiveBlock::Times],
                vec![ActiveBlock::Scramble, ActiveBlock::Stats, ActiveBlock::Main],
            ],
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
            time.gen_stats(&self.times);
            self.times.push(*time);
        }
        Ok(())
    }

    pub fn write_times(&self) -> Result<(), Box<dyn Error>> {
        let write_data: Vec<u8> = self
            .times
            .iter()
            .flat_map(|v| {
                format!("{}\n", v.to_string())
                    .bytes()
                    .collect::<Vec<u8>>()
            })
            .collect();
        fs::write(&self.path, write_data)?;
        Ok(())
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
        let i = match self.times_state.selected() {
            Some(i) => {
                if i >= self.times.len() - 1 {
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
        let i = match self.times_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.times.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.times_state.select(Some(i));
    }

    pub fn on_tick(&self) {
        ()
    }
}
