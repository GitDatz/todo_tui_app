use tui::{
  layout::{ Alignment, Constraint },
  style::{ Color, Modifier, Style },
  text::{ Span, Spans },
  widgets::{ Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table },
};

use crate::constants;
use crate::data::task as data;

pub fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(constants::HOME_DESCRIPTION_ROW_1)]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(constants::HOME_DESCRIPTION_ROW_2)]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(constants::HOME_DESCRIPTION_ROW_3)]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(constants::HOME_PAGE_TITLE)
            .border_type(BorderType::Plain),
    );
    home
}

pub fn render_tasks<'a>(task_list_state: &ListState, task_list: Vec<data::Task>) -> (List<'a>, Table<'a>) {
    let tasks = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(constants::TASKS_PAGE_TITLE)
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

    let selected_task = task_list
        .get(
            task_list_state
                .selected()
                .expect("no task selected!"),
        )
        .expect("exists")
        .clone();

    let task_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_task.name)),
        Cell::from(Span::raw(selected_task.description)),
        Cell::from(Span::raw(selected_task.date_added.to_string())),
        ])])
        .header(Row::new(vec![
            Cell::from(Span::styled(
                constants::DETAIL_NAME_TITLE,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                constants::DETAIL_DESCRIPTION_TITLE,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                constants::DETAIL_CREATED_AT_TITLE,
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(constants::DETAILS_PAGE_TITLE)
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
        ]);

    (list, task_detail)
}
