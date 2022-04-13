use std::fmt::Debug;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
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
            memory_usage_percentage: 0.0,
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

        if crossterm::event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    println!("Quit wanted by user.");
                    quit.store(true, Ordering::Relaxed);
                    break;
                }
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
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(5), // title
                Constraint::Percentage(20),// system information
                Constraint::Percentage(75),// processes
            ]
                .as_ref(),
        )
        .split(size);

    // system info area is divided in two ares
    // one for dynamic data on left, and one for static data on right
    let sys_info_areas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
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
    draw_sys_info_left(f, sys_info_areas[0], system_info_dynamic.cpu_usage,
                       system_info_dynamic.memory_usage_percentage,
                       system_info_dynamic.page_memory_usage_percentage);
    draw_sys_info_right(f, system_info_static, system_info_dynamic.total_tasks_count, system_info_dynamic.uptime, sys_info_areas[1]);
    // f.render_widget(process_block, main_areas[2]);
    draw_logo_block(f, main_areas[2]);
}

fn draw_title<B: Backend>(f: &mut Frame<'_, B>, hostname: String, area: Rect) {
    let title = format!("riptop on {}", hostname);
    let text = vec![
        Span::styled("riptop - ", Style::default().fg(Color::Yellow)),
        Span::styled(hostname, Style::default().fg(Color::White)),
    ];
    let title_block = Block::default()
        .title(text.clone())
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Blue));
    f.render_widget(title_block, area);
}

fn draw_sys_info_left<B: Backend>(f: &mut Frame<'_, B>, area: Rect, cpu: f64, mem: f64, pge: u64) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(area);
    let gauges_lines = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(columns[1]);
    let labels = vec![
        Spans::from(Span::styled("CPU: ", Style::default().fg(Color::Yellow))),
        Spans::from(Span::styled("MEM: ", Style::default().fg(Color::Yellow))),
        Spans::from(Span::styled("PGE: ", Style::default().fg(Color::Yellow))),
    ];

    let sys_static_labels_paragraph = Paragraph::new(labels.clone())
        .alignment(Alignment::Left);

    let cpu_label = format!("{:.2}%", cpu);
    let mem_label = format!("{}%", mem);
    let cpu_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .label(cpu_label.clone())
        .ratio(cpu.clone());
    let mem_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .label(mem_label.clone())
        .ratio(mem.clone() as f64);
    let pge_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .label(cpu_label.clone())
        .percent(11);
    // let mem_gauge = Gauge::default()
    //     .block(sys_dyn_block)
    //     .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
    //     .label(mem_label)
    //     .ratio(0.8);

    f.render_widget(sys_static_labels_paragraph, columns[0]);
    f.render_widget(cpu_gauge, gauges_lines[0]);
    f.render_widget(mem_gauge, gauges_lines[1]);
    f.render_widget(pge_gauge, gauges_lines[2]);
}

fn draw_sys_info_right<B: Backend>(f: &mut Frame<'_, B>, system_info_static: SystemInfoStaticData,
                                   task_count: usize, uptime: u64, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(area);

    let labels = vec![
        Spans::from(Span::styled("Tasks: ", Style::default().fg(Color::Yellow))),
        Spans::from(Span::styled("Size: ", Style::default().fg(Color::Yellow))),
        Spans::from(Span::styled("Uptime: ", Style::default().fg(Color::Yellow))),
        Spans::from(Span::styled("Proc: ", Style::default().fg(Color::Yellow))),
    ];

    let uptime_text = format_uptime(uptime);



    let values = vec![
        //Span::from("Proc: ", Style::default().fg(Color::Yellow)),
        Spans::from(task_count.to_string()),
        Spans::from(system_info_static.total_memory.to_string()),
        Spans::from(uptime_text),
        Spans::from(system_info_static.processor_name),
    ];


    let sys_static_paragraph = Paragraph::new(labels.clone())
        .alignment(Alignment::Left);
    let sys_static_values_paragraph = Paragraph::new(values.clone())
        .alignment(Alignment::Left);

    f.render_widget(sys_static_paragraph, columns[0]);
    f.render_widget(sys_static_values_paragraph, columns[1]);
}

fn format_uptime(uptime: u64) -> String {
    let seconds = uptime % 60;
    let minutes = (uptime / 60) % 60;
    let hours = (uptime / 60) / 60;

    let uptime_text = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    uptime_text
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
