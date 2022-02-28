pub mod services;

use crate::services::SystemDataService;

fn main() {
    println!("Hello, world!");

    let mut system_data_service = SystemDataService::new();
    system_data_service.acquire();

    print!("CPU NAME: {}", system_data_service.static_data().computer_name);
    // print!("CPU COUNT: {}", system_data_service.cpu_count().unwrap());


}
