extern crate sysinfo;

use sysinfo::{ProcessExt, ProcessorExt, System, SystemExt};

pub struct SystemInfoStaticData{
    pub processor_name: String,
    pub computer_name: String,
    pub cpu_count: usize,
    pub total_memory: u64,
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

    pub fn cpu_count(self) -> Option<usize> {
        self.system.physical_core_count()
    }

    pub fn static_data(self)-> SystemInfoStaticData {
        SystemInfoStaticData{
            processor_name: self.system.global_processor_info().brand().to_string(),
            computer_name: self.system.host_name().unwrap().to_string(),
            cpu_count: self.system.physical_core_count().unwrap(),
            total_memory: self.system.total_memory()
        }
    }
}

impl Default for SystemDataService {
    fn default() -> Self {
        Self::new()
    }
}
