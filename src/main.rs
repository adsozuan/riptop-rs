pub mod services;

use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crate::services::{create_acquisition_thread, SystemDataService};

fn main() {
    let mut quit = AtomicBool::new(false);
    let (system_info_tx, system_info_rx) = mpsc::channel();


    let mut system_data_service = SystemDataService::new();
    println!("Static data: {:?}", system_data_service.static_data());

    let _acquisition_thread = create_acquisition_thread(quit, system_info_tx, system_data_service);


    println!("Dynamic data: {:?}", system_info_rx.recv().unwrap());
}
