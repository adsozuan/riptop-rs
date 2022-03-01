extern crate sysinfo;

use sysinfo::{ProcessExt, ProcessorExt, ProcessStatus, System, SystemExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct SystemInfoStaticData {
    pub processor_name: String,
    pub computer_name: String,
    pub cpu_count: usize,
    pub total_memory: u64,
}

#[derive(Debug)]
pub struct SystemInfoDynamicData {
    pub cpu_usage: f64,
    pub memory_usage_percentage: u64,
    pub page_memory_usage_percentage: u64,
    pub total_tasks_count: usize,
    pub running_tasks_count: usize,
    pub uptime: u64,
}


pub struct SystemDataService {
    system: System,
}

impl SystemDataService {
    pub fn new() -> SystemDataService {
        SystemDataService {
            system: System::new_all(),
        }
    }

    pub fn acquire(&mut self) {
        self.system.refresh_all();
    }

    pub fn cpu_count(&self) -> Option<usize> {
        self.system.physical_core_count()
    }

    pub fn static_data(&self) -> SystemInfoStaticData {
        SystemInfoStaticData {
            processor_name: self.system.global_processor_info().brand().to_string(),
            computer_name: self.system.host_name().unwrap().to_string(),
            cpu_count: self.system.physical_core_count().unwrap(),
            total_memory: self.system.total_memory(),
        }
    }

    pub fn dynamic_data(&self) -> SystemInfoDynamicData {
        SystemInfoDynamicData {
            cpu_usage: self.system.load_average().one,
            memory_usage_percentage: self.system.used_memory(),
            page_memory_usage_percentage: self.system.used_swap(),
            total_tasks_count: self.system.processes().len(),
            running_tasks_count: self.running_tasks_count(),
            uptime: self.system.uptime(),
        }
    }

    fn running_tasks_count(&self) -> usize {
        let mut count: usize = 0;
        for (_pid, process) in self.system.processes() {
            if process.status() == ProcessStatus::Run {
                count = count + 1;
            }
        }
        count
    }
}

impl Default for SystemDataService {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_acquisition_thread(quit: Arc<AtomicBool>, system_info_tx: std::sync::mpsc::Sender<SystemInfoDynamicData>,
                                 mut system_data_service: SystemDataService) {
    thread::spawn(move || {
        loop {
            if quit.load(Ordering::Relaxed) {
                println!("Stop acquiring...");
                break;
            }
            system_data_service.acquire();
            system_info_tx.send(system_data_service.dynamic_data());
            thread::sleep(Duration::from_millis(500));
        }
    });
}
