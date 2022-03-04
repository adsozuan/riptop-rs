pub mod services;
mod ui;

use std::error::Error;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool};
use std::sync::Arc;

use crate::services::{create_acquisition_thread, SystemDataService};
use crate::ui::run_ui;

fn main() -> Result<(), Box<dyn Error>> {
    let quit = Arc::new(AtomicBool::new(false));
    let quit_write = quit.clone();
    let (system_info_tx, system_info_rx) = mpsc::channel();


    let mut system_data_service = SystemDataService::new();
    let system_info_static_data = system_data_service.static_data();

    let _acquisition_thread = create_acquisition_thread(quit, system_info_tx, system_data_service);



    // create ui and run it
    let res = run_ui(quit_write, system_info_static_data, system_info_rx);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}


