use sysinfo::{ComponentExt, CpuExt, DiskExt, System, SystemExt};

use crate::types::HwInfo;

pub fn get_hw_info() -> HwInfo {
    let mut sys = System::new_all();

    sys.refresh_all();

    let mut hw_info = HwInfo {
        uptime: sys.uptime(),
        os_name: sys.long_os_version().unwrap_or("".to_string()),
        cpu_name: String::from(""),
        cpu_freq: Vec::new(),
        cpu_usage: Vec::new(),
        cpu_temp: Vec::new(),
        cpu_cores: (0, 0),
        ram_total: 0,
        ram_used: 0,
        ram_free: 0,
        ram_percentage: 0.0,
        swap_total: 0,
        swap_used: 0,
        swap_free: 0,
        swap_percentage: 0.0,
        disks: Vec::new(),
    };

    sys.refresh_all();

    // Wait for 200ms to get the CPU usage
    let delay = std::time::Duration::from_millis(200);

    std::thread::sleep(delay);

    sys.refresh_all();

    let cpus = sys.cpus();

    let components = sys.components();

    hw_info.cpu_name = cpus[0].brand().to_string();
    hw_info.cpu_cores = (sys.physical_core_count().unwrap_or(0), cpus.len());

    for cpu in cpus {
        hw_info.cpu_freq.push(cpu.frequency());
        hw_info.cpu_usage.push(cpu.cpu_usage());
    }

    for component in components {
        hw_info.cpu_temp.push(component.temperature());
    }

    hw_info.ram_total = sys.total_memory();
    hw_info.ram_used = sys.used_memory();
    hw_info.ram_free = sys.free_memory();
    hw_info.ram_percentage = sys.used_memory() as f32 / sys.total_memory() as f32;

    hw_info.swap_total = sys.total_swap();
    hw_info.swap_used = sys.used_swap();
    hw_info.swap_free = sys.free_swap();
    hw_info.swap_percentage = sys.used_swap() as f32 / sys.total_swap() as f32;

    for disk in sys.disks() {
        hw_info.disks.push((
            format!("{:?}", disk.kind()),
            disk.total_space(),
            disk.total_space() - disk.available_space(),
            disk.available_space(),
            (disk.total_space() - disk.available_space()) as f32 / disk.total_space() as f32,
        ));
    }

    hw_info
}
