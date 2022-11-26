use super::app::*;
use ::crossterm::event::{self, Event, KeyCode};
use std::{
    error::Error,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Color, Style},
    Frame, Terminal,
};

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut app = App::new(Duration::from_millis(100));
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = app
            .tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => app.timer.space_press(),
                    _ => (),
                }
            }
        }       
        if last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // define chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(7),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    // render left side
    render_help_and_tools(f, app, left_chunks[0]);
    render_time(f, app, left_chunks[1]);
    let block = Block::default().title("Times").borders(Borders::ALL);
    f.render_widget(block, left_chunks[2]);

    // render right side
    let block = Block::default().title("Scramble").borders(Borders::ALL);
    f.render_widget(block, right_chunks[0]);
    let block = Block::default().title("Main").borders(Borders::ALL);
    f.render_widget(block, right_chunks[1]);
}

fn render_help_and_tools<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout_chunk);

    let block = Block::default().title("Tools").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Help").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

fn render_time<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let text = format!("\n\n{}", app.timer.text());

    let block = Paragraph::new(text)
        .block(Block::default().title("Timer").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(block, layout_chunk);
}
