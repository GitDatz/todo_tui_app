use chrono::{ Datelike, Utc };
use crossterm::{
    event::{ self, Event as CtEvent, KeyCode },
    terminal::{ disable_raw_mode, enable_raw_mode }
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{ Duration, Instant };
use tui::{
    backend::CrosstermBackend,
    layout::{ Alignment, Constraint, Direction, Layout },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    widgets::{ Block, BorderType, Borders, Paragraph, Tabs },
    Terminal,
};

enum Event<I> {
    Press(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("enable raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("polling") {
                if let CtEvent::Key(key) = event::read().expect("reading CtEvent") {
                    tx.send(Event::Press(key)).expect("sending Event::Press");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Tasks", "Quit"];

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

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

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

            let tabs = Tabs::new(menu)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Press(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}

fn get_current_date() -> String {
    let now = Utc::now();
    let date = format!(" {:04}-{:02}-{:02} ", now.year(), now.month(), now.day());
    date
}
