use tui::{
  layout::{ Alignment, Constraint },
  style::{ Color, Modifier, Style },
  text::{ Span, Spans },
  widgets::{ Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Table },
};

use crate::data::task as data;

pub fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Home of TODO TUI app")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Add tasks for today!")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'q' to quit.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

pub fn render_tasks<'a>(task_list_state: &ListState, task_list: Vec<data::Task>) -> List<'a> {
    let tasks = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(" Tasks ")
        .border_type(BorderType::Plain);

    let items: Vec<_> = task_list
        .iter()
        .map(|task| {
            ListItem::new(Spans::from(vec![Span::styled(
                task.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(tasks).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    list
}
