use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossterm::event::{Event, KeyCode, DisableMouseCapture, EnableMouseCapture};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::Block;

struct MainWidget {}

pub fn run_ui(quit: Arc<AtomicBool>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                println!("Quit wanted by user.");
                quit.store(true, Ordering::Relaxed);
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    return Ok(());
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let main_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(20), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let sys_info_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_areas[1]);

    let title_block = Block::default()
        .title(vec![
            Span::styled("riptop", Style::default().fg(Color::Yellow)),
            Span::from("Ze computer"),
        ])
        .style(Style::default().bg(Color::Blue));

    let sys_dyn_block = Block::default()
        .title(vec![
            Span::styled("CPU", Style::default().fg(Color::Yellow)),
            Span::from("& Cie"),
        ])
        .style(Style::default());//.bg(Color::Green));

    let sys_static_block = Block::default()
        .title(vec![
            Span::styled("CPU TYPE", Style::default().fg(Color::Yellow)),
            Span::from("& Cie"),
        ])
        .style(Style::default());//.bg(Color::Red));

    let process_block = Block::default()
        .title(vec![
            Span::styled("Processes", Style::default().fg(Color::White)),
            Span::from("---"),
        ])
        .style(Style::default());//.bg(Color::Gray));

    f.render_widget(title_block, main_areas[0]);
    f.render_widget(sys_dyn_block, sys_info_areas[0]);
    f.render_widget(sys_static_block, sys_info_areas[1]);
    f.render_widget(process_block, main_areas[2]);
}

