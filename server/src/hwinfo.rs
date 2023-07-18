use sysinfo::{ComponentExt, CpuExt, DiskExt, System, SystemExt};

use crate::types::HwInfo;

fn get_gb_string(used_bytes: u64, total_bytes: u64, percent: f64) -> String {
    if used_bytes == 0 && total_bytes == 0 {
        return String::from("0.00 GB / 0.00 GB (0.00%)");
    }

    let used_gb = used_bytes as f32 / 1024.0 / 1024.0 / 1024.0;
    let total_gb = total_bytes as f32 / 1024.0 / 1024.0 / 1024.0;

    format!(
        "{:.2} GB / {:.2} GB ({:.2}%)",
        used_gb,
        total_gb,
        percent * 100.0
    )
}

/// Get the hardware information of the system.
pub fn get_hw_info() -> HwInfo {
    let mut sys = System::new_all();

    sys.refresh_all();

    let mut hw_info = HwInfo {
        uptime: sys.uptime(),
        os_name: sys.long_os_version().unwrap_or("".to_string()),
        cpu_name: String::from(""),
        cpu_freq: Vec::new(),
        avg_cpu_freq: 0,
        cpu_usage: Vec::new(),
        avg_cpu_usage: 0.0,
        cpu_temp_c: Vec::new(),
        avg_cpu_temp_c: 0.0,
        cpu_temp_f: Vec::new(),
        avg_cpu_temp_f: 0.0,
        cpu_cores: (0, 0),
        ram_total: 0,
        ram_used: 0,
        ram_free: 0,
        ram_percentage: 0.0,
        ram_readable_str: String::from(""),
        swap_total: 0,
        swap_used: 0,
        swap_free: 0,
        swap_percentage: 0.0,
        swap_readable_str: String::from(""),
        disks: Vec::new(),
    };

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
        hw_info.cpu_temp_c.push(component.temperature());
        hw_info
            .cpu_temp_f
            .push(component.temperature() * 9.0 / 5.0 + 32.0);
    }

    hw_info.avg_cpu_freq = hw_info.cpu_freq.iter().sum::<u64>() / hw_info.cpu_freq.len() as u64;
    hw_info.avg_cpu_usage = hw_info.cpu_usage.iter().sum::<f32>() / hw_info.cpu_usage.len() as f32;
    hw_info.avg_cpu_temp_c =
        hw_info.cpu_temp_c.iter().sum::<f32>() / hw_info.cpu_temp_c.len() as f32;
    hw_info.avg_cpu_temp_f =
        hw_info.cpu_temp_f.iter().sum::<f32>() / hw_info.cpu_temp_f.len() as f32;

    hw_info.ram_total = sys.total_memory();
    hw_info.ram_used = sys.used_memory();
    hw_info.ram_free = sys.free_memory();
    hw_info.ram_percentage = sys.used_memory() as f32 / sys.total_memory() as f32;
    hw_info.ram_readable_str = get_gb_string(
        sys.used_memory(),
        sys.total_memory(),
        sys.used_memory() as f64 / sys.total_memory() as f64,
    );

    hw_info.swap_total = sys.total_swap();
    hw_info.swap_used = sys.used_swap();
    hw_info.swap_free = sys.free_swap();
    hw_info.swap_percentage = sys.used_swap() as f32 / sys.total_swap() as f32;
    hw_info.swap_readable_str = get_gb_string(
        sys.used_swap(),
        sys.total_swap(),
        sys.used_swap() as f64 / sys.total_swap() as f64,
    );

    for disk in sys.disks() {
        let percent =
            (disk.total_space() - disk.available_space()) as f32 / disk.total_space() as f32;

        hw_info.disks.push((
            format!("{:?}", disk.kind()),
            disk.total_space(),
            disk.total_space() - disk.available_space(),
            disk.available_space(),
            percent,
            get_gb_string(
                disk.total_space() - disk.available_space(),
                disk.total_space(),
                percent as f64,
            ),
        ));
    }

    hw_info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hwinfo() {
        let hw_info = get_hw_info();

        assert!(hw_info.uptime > 0);
        assert!(!hw_info.os_name.is_empty());
        assert!(!hw_info.cpu_name.is_empty());
        assert!(!hw_info.cpu_freq.is_empty());
        assert!(hw_info.avg_cpu_freq > 0);
        assert!(!hw_info.cpu_usage.is_empty());
        assert!(hw_info.avg_cpu_usage > 0.0);
        assert!(hw_info.cpu_cores.0 > 0);
        assert!(hw_info.cpu_cores.1 > 0);
        assert!(hw_info.ram_total > 0);
        assert!(hw_info.ram_used > 0);
        assert!(hw_info.ram_free > 0);
        assert!(hw_info.ram_percentage > 0.0);
        assert!(!hw_info.ram_readable_str.is_empty());
        assert!(!hw_info.disks.is_empty());
        assert!(!hw_info.swap_readable_str.is_empty());
    }

    #[test]
    fn test_get_gb_string() {
        assert_eq!(
            get_gb_string(0, 0, 0.0),
            String::from("0.00 GB / 0.00 GB (0.00%)")
        );
        assert_eq!(
            get_gb_string(1, 1, 1.0),
            String::from("0.00 GB / 0.00 GB (100.00%)")
        );
        assert_eq!(
            get_gb_string(1024, 1024, 1.0),
            String::from("0.00 GB / 0.00 GB (100.00%)")
        );
        assert_eq!(
            get_gb_string(1024 * 1024, 1024 * 1024, 1.0),
            String::from("0.00 GB / 0.00 GB (100.00%)")
        );
        assert_eq!(
            get_gb_string(512 * 1024 * 1024, 1024 * 1024 * 1024, 1.0),
            String::from("0.50 GB / 1.00 GB (100.00%)")
        );
        assert_eq!(
            get_gb_string(1024 * 1024 * 1024 * 1024, 1024 * 1024 * 1024 * 1024, 1.0),
            String::from("1024.00 GB / 1024.00 GB (100.00%)")
        );
    }
}
