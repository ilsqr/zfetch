use sysinfo::System;
use std::fs;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub username: String,
    pub hostname: String,
    pub os_name: String,
    pub kernel_version: String,
    pub uptime: u64,
    pub shell: String,
    pub cpu_info: String,
    pub gpu_info: String,
    pub memory_info: MemoryInfo,
    pub desktop_environment: String,
    pub terminal: String,
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            username: Self::get_username(),
            hostname: Self::get_hostname(),
            os_name: Self::get_os_name(),
            kernel_version: Self::get_kernel_version(),
            uptime: System::uptime(),
            shell: Self::get_shell(),
            cpu_info: Self::get_cpu_info(&sys),
            gpu_info: Self::get_gpu_info(),
            memory_info: Self::get_memory_info(&sys),
            desktop_environment: Self::get_desktop_environment(),
            terminal: Self::get_terminal(),
        }
    }

    fn get_username() -> String {
        std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    }

    fn get_hostname() -> String {
        fs::read_to_string("/etc/hostname")
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string()
    }

    fn get_os_name() -> String {
        if let Ok(contents) = fs::read_to_string("/etc/os-release") {
            for line in contents.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return line
                        .split('=')
                        .nth(1)
                        .unwrap_or("Unknown")
                        .trim_matches('"')
                        .to_string();
                }
            }
        }
        "Unknown Linux".to_string()
    }

    fn get_kernel_version() -> String {
        fs::read_to_string("/proc/version")
            .unwrap_or_else(|_| "unknown".to_string())
            .split_whitespace()
            .nth(2)
            .unwrap_or("unknown")
            .to_string()
    }

    fn get_shell() -> String {
        std::env::var("SHELL")
            .unwrap_or_else(|_| "unknown".to_string())
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string()
    }

    fn get_cpu_info(sys: &System) -> String {
        let cpus = sys.cpus();
        if let Some(cpu) = cpus.first() {
            let cpu_name = cpu.brand().trim();
            let cpu_count = cpus.len();
            format!("{} ({} cores)", cpu_name, cpu_count)
        } else {
            "Unknown CPU".to_string()
        }
    }

    fn get_memory_info(sys: &System) -> MemoryInfo {
        let total = sys.total_memory();
        let used = sys.used_memory();
        let available = sys.available_memory();

        MemoryInfo {
            total,
            used,
            available,
        }
    }

    fn get_gpu_info() -> String {
        // Try to get GPU info from lspci first
        if let Ok(output) = std::process::Command::new("lspci")
            .arg("-mm")
            .output()
        {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.to_lowercase().contains("vga") || 
                       line.to_lowercase().contains("3d") ||
                       line.to_lowercase().contains("display") {
                        // Extract GPU name from lspci output
                        let parts: Vec<&str> = line.split('"').collect();
                        if parts.len() >= 4 {
                            let vendor = parts[3];
                            let device = parts[5];
                            return format!("{} {}", vendor, device);
                        } else if parts.len() >= 2 {
                            return parts[1].to_string();
                        }
                    }
                }
            }
        }

        // Try glxinfo as fallback
        if let Ok(output) = std::process::Command::new("glxinfo")
            .arg("-B")
            .output()
        {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("OpenGL renderer string:") {
                        if let Some(gpu_name) = line.split(':').nth(1) {
                            return gpu_name.trim().to_string();
                        }
                    }
                }
            }
        }

        // Try nvidia-smi for NVIDIA GPUs
        if let Ok(output) = std::process::Command::new("nvidia-smi")
            .arg("--query-gpu=gpu_name")
            .arg("--format=csv,noheader,nounits")
            .output()
        {
            if output.status.success() {
                let gpu_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !gpu_name.is_empty() {
                    return gpu_name;
                }
            }
        }

        // Try reading from /proc/driver/nvidia if available
        if let Ok(contents) = fs::read_to_string("/proc/driver/nvidia/gpus") {
            if !contents.is_empty() {
                return "NVIDIA GPU (details unavailable)".to_string();
            }
        }

        // Try reading from /sys/class/drm for AMD/Intel GPUs
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        if name_str.starts_with("card") && !name_str.contains('-') {
                            let device_path = path.join("device/vendor");
                            if let Ok(vendor) = fs::read_to_string(&device_path) {
                                let vendor_id = vendor.trim();
                                match vendor_id {
                                    "0x1002" => return "AMD GPU".to_string(),
                                    "0x8086" => return "Intel GPU".to_string(),
                                    "0x10de" => return "NVIDIA GPU".to_string(),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        "Unknown GPU".to_string()
    }

    fn get_desktop_environment() -> String {
        // Check various environment variables for DE detection
        let desktop_vars = [
            "XDG_CURRENT_DESKTOP",
            "DESKTOP_SESSION",
            "GDMSESSION",
        ];

        for var in &desktop_vars {
            if let Ok(value) = std::env::var(var) {
                if !value.is_empty() {
                    return value;
                }
            }
        }

        // Check for specific DE processes using which command (safer than pgrep)
        if std::process::Command::new("which")
            .arg("gnome-shell")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return "GNOME".to_string();
        }

        if std::process::Command::new("which")
            .arg("kwin")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return "KDE".to_string();
        }

        "Unknown".to_string()
    }

    fn get_terminal() -> String {
        std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string())
    }

    pub fn format_uptime(&self) -> String {
        let days = self.uptime / 86400;
        let hours = (self.uptime % 86400) / 3600;
        let minutes = (self.uptime % 3600) / 60;

        if days > 0 {
            format!("{} days, {} hours, {} minutes", days, hours, minutes)
        } else if hours > 0 {
            format!("{} hours, {} minutes", hours, minutes)
        } else {
            format!("{} minutes", minutes)
        }
    }

    pub fn format_memory(&self) -> String {
        let total_gb = self.memory_info.total as f64 / 1_073_741_824.0;
        let used_gb = self.memory_info.used as f64 / 1_073_741_824.0;
        let percentage = (used_gb / total_gb * 100.0) as u8;
        
        format!("{:.1} GB / {:.1} GB ({}%)", used_gb, total_gb, percentage)
    }
}
