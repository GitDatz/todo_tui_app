use chrono::{ Datelike, Utc };
use crossterm::{
    event::{ KeyCode },
    terminal::{ disable_raw_mode, enable_raw_mode }
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{ Alignment, Constraint, Direction, Layout },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    widgets::{ Block, BorderType, Borders, ListState, Paragraph, Tabs },
    Terminal,
};

use crate::constants;
use crate::presenter;
use crate::types;
use crate::ui::pages as pages;

pub fn render_main_ui(receiver: std::sync::mpsc::Receiver<types::Event<crossterm::event::KeyEvent>>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("enable raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec![constants::HOME_TAB_TITLE, constants::TASKS_TAB_TITLE, constants::QUIT_TAB_TITLE];
    let mut current_page = types::Page::Home;
    let mut task_list_state = ListState::default();
    task_list_state.select(Some(0));

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

          let description = Paragraph::new(constants::APP_TITLE)
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

          match current_page {
              types::Page::Home => rect.render_widget(pages::render_home(), chunks[1]),
              types::Page::Tasks => {
                  let tasks_chunks = Layout::default()
                      .direction(Direction::Horizontal)
                      .constraints(
                          [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                      )
                      .split(chunks[1]);
                  let (side_bar, main_window) = pages::render_tasks(&task_list_state, presenter::read_test_db().expect("Could not parse database"));
                  rect.render_stateful_widget(side_bar, tasks_chunks[0], &mut task_list_state);
                  rect.render_widget(main_window, tasks_chunks[1]);
              }
          }
        })?;
        match receiver.recv()? {
            types::Event::Press(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().expect("disable_raw_mode");
                    terminal.show_cursor().expect("terminal show_cursor()");
                    break;
                }
                KeyCode::Char('h') => current_page = types::Page::Home,
                KeyCode::Char('t') => current_page = types::Page::Tasks,
                KeyCode::Down => {
                    if let Some(selected) = task_list_state.selected() {
                        let nr_of_tasks = presenter::read_test_db().expect("can not fetch tasks list").len();
                        if selected >= nr_of_tasks - 1 {
                            task_list_state.select(Some(0));
                        } else {
                            task_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = task_list_state.selected() {
                        let nr_of_tasks = presenter::read_test_db().expect("can not fetch tasks list").len();
                        if selected > 0 {
                            task_list_state.select(Some(selected - 1));
                        } else {
                            task_list_state.select(Some(nr_of_tasks - 1));
                        }
                    }
                }
                _ => {}
            },
            types::Event::Tick => {}
        }
    }
    Ok(())
}

fn get_current_date() -> String {
    let now = Utc::now();
    let date = format!(" {:04}-{:02}-{:02} ", now.year(), now.month(), now.day());
    date
}
