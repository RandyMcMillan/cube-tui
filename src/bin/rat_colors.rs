use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use tui::style::Modifier;
use tui::{backend::CrosstermBackend, layout::Rect, style::Color, widgets::Block, Terminal};
use tui::style::Style;

fn color_name(color: Color) -> &'static str {
    match color {
        Color::Black => "Black",
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Yellow => "Yellow",
        Color::Blue => "Blue",
        Color::Magenta => "Magenta",
        Color::Cyan => "Cyan",
        Color::White => "White",
        Color::Gray => "Gray",
        Color::DarkGray => "DarkGray",
        Color::LightRed => "LightRed",
        Color::LightGreen => "LightGreen",
        Color::LightYellow => "LightYellow",
        Color::LightBlue => "LightBlue",
        Color::LightMagenta => "LightMagenta",
        Color::LightCyan => "LightCyan",
        Color::Reset => "Reset", // Added Reset
        Color::Rgb(r, g, b) => {
            return Box::leak(format!("Rgb({}, {}, {})", r, g, b).into_boxed_str());
        }
        Color::Indexed(i) => {
            return Box::leak(format!("Indexed({})", i).into_boxed_str());
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let colors = [
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::Gray,
            Color::DarkGray,
            //Color::LightRed,
            //Color::LightGreen,
            //Color::LightYellow,
            //Color::LightBlue,
            //Color::LightMagenta,
            //Color::LightCyan,
            //Color::LightWhite,
        ];

        let mut x = 0;
        let mut y = 0;
        let width = 10;
        let height = 2;

        for index in 0..colors.len() {
            for color in colors {
                let block = Block::default()
                    .style(Style::default().fg(colors[index]).bg(color))
                    .title(color_name(color));
                let area = Rect::new(x, y, width, height);

                f.render_widget(block, area);

                x += width;
                if x + width > f.size().width {
                    x = 0;
                    y += height;
                }
            }
            y += height; // Move to the next row after each color set
            x=0; // reset x to zero.
        }
    })?;

    disable_raw_mode()?;
    std::io::stdout().flush()?;
    Ok(())
}
