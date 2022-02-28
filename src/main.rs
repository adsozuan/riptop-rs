pub mod services;
mod ui;

use std::error::Error;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, thread};
use std::time::Duration;
use crossterm::event::{Event, KeyCode, DisableMouseCapture, EnableMouseCapture};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use crate::services::{create_acquisition_thread, SystemDataService};
use crate::ui::run_ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut quit = AtomicBool::new(false);
    let (system_info_tx, system_info_rx) = mpsc::channel();


    let mut system_data_service = SystemDataService::new();
    println!("Static data: {:?}", system_data_service.static_data());

    let _acquisition_thread = create_acquisition_thread(quit, system_info_tx, system_data_service);


    println!("Dynamic data: {:?}", system_info_rx.recv().unwrap());

    // create ui and run it
    let res = run_ui();

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}


