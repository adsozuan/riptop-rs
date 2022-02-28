use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::Block;

struct MainWidget {


}

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let main_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let top_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_areas[0]);

    let block = Block::default()
        .title(vec![
            Span::styled("With", Style::default().fg(Color::Yellow)),
            Span::from(" background"),
        ])
        .style(Style::default().bg(Color::Green));

    f.render_widget(block, top_areas[0]);

}