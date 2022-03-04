use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Gauge, Paragraph};
use tui::{Frame, Terminal};

use crate::services::{SystemInfoDynamicData, SystemInfoStaticData};

struct MainWidget {}

pub fn run_ui(
    quit: Arc<AtomicBool>,
    system_info_static_data: SystemInfoStaticData,
    system_info_rx: mpsc::Receiver<SystemInfoDynamicData>,
) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let mut incoming_info: SystemInfoDynamicData = SystemInfoDynamicData {
            cpu_usage: 0.0,
            memory_usage_percentage: 0,
            page_memory_usage_percentage: 0,
            total_tasks_count: 0,
            running_tasks_count: 0,
            uptime: 0,
        };
        let res = system_info_rx.recv();
        match res {
            Ok(sys_info) => incoming_info = sys_info,
            Err(_) => todo!(),
        }
        terminal.draw(|f| ui(f,
                             system_info_static_data.clone(),
                             incoming_info))?;

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

fn ui<B: Backend>(f: &mut Frame<B>, system_info_static: SystemInfoStaticData,
                  system_info_dynamic: SystemInfoDynamicData) {
    let size = f.size();

    let main_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(70),
            ]
                .as_ref(),
        )
        .split(size);

    let sys_info_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_areas[1]);

    let process_block = Block::default()
        .title(vec![
            Span::styled("Processes", Style::default().fg(Color::White)),
            Span::from("---"),
        ])
        .borders(Borders::TOP)
        .style(Style::default());

    draw_title(f, system_info_static.computer_name.clone(), main_areas[0]);
    draw_sys_info_dynamic(f, sys_info_areas[0], system_info_dynamic.cpu_usage);
    draw_sys_info_static(f, system_info_static, sys_info_areas[1]);
    // f.render_widget(process_block, main_areas[2]);
    draw_logo_block(f, main_areas[2]);
}

fn draw_title<B: Backend>(f: &mut Frame<'_, B>, hostname: String, area: Rect) {
    let title_block = Block::default()
        .title(vec![
            Span::styled("riptop on ", Style::default().fg(Color::Yellow)),
            Span::from(hostname),
        ])
        .style(Style::default().bg(Color::Blue));
    f.render_widget(title_block, area);
}

fn draw_sys_info_dynamic<B: Backend>(f: &mut Frame<'_, B>, area: Rect, cpu: f64) {
    let sys_dyn_block = Block::default()
        .title(vec![
            Span::styled("CPU: ", Style::default().fg(Color::Yellow)),
            Span::from("& Cie"),
        ])
        .borders(Borders::RIGHT)
        .style(Style::default()); //.bg(Color::Green));

    let cpu_label = format!("{:.2}%", cpu);
    let cpu_gauge = Gauge::default()
        .block(sys_dyn_block)
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .label(cpu_label)
        .ratio(0.8);

    f.render_widget(cpu_gauge, area);
}

fn draw_sys_info_static<B: Backend>(f: &mut Frame<'_, B>, system_info_static: SystemInfoStaticData, area: Rect) {
    let sys_static_block = Block::default()
        .title(vec![
            Span::styled("Proc: ", Style::default().fg(Color::Yellow)),
            Span::from(system_info_static.processor_name),
            Span::styled("\n", Style::default().fg(Color::Yellow)),
            Span::from(system_info_static.cpu_count.to_string()),
        ])
        .style(Style::default());

    f.render_widget(sys_static_block, area);
}

pub const BANNER: &str = r#"

  _____  _____ _____ _______ ____  _____  
 |  __ \|_   _|  __ \__   __/ __ \|  __ \ 
 | |__) | | | | |__) | | | | |  | | |__) |
 |  _  /  | | |  ___/  | | | |  | |  ___/ 
 | | \ \ _| |_| |      | | | |__| | |     
 |_|  \_\_____|_|      |_|  \____/|_|     
                                          
                                          

"#;

fn draw_logo_block<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
    // Banner text with correct styling
    let text = format!("{}\n v with ♥ in Rust ", BANNER, );
    let mut text = Text::from(text);

    // Contains the banner
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}
