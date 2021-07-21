use tui::{
  layout::{ Alignment },
  style::{ Color, Style },
  text::{ Span, Spans },
  widgets::{ Block, BorderType, Borders, Paragraph },
};

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