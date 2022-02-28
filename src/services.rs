extern crate sysinfo;

use sysinfo::{ProcessExt, System, SystemExt};

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
}

impl Default for SystemDataService {
    fn default() -> Self {
        Self::new()
    }
}
