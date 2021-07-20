use chrono::{ Datelike, Utc };
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{ Alignment, Constraint, Direction, Layout },
    style::{ Color, Style },
    widgets::{ Block, BorderType, Borders, Paragraph },
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let description = Paragraph::new("TODO TUI - what to do today?")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title(get_current_date())
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(description, chunks[0]);
        })?;
    }
}

fn get_current_date() -> String {
    let now = Utc::now();
    let date = format!(" {:04}-{:02}-{:02} ", now.year(), now.month(), now.day());
    date
}
