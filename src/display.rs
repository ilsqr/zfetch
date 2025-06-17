use colored::*;
use crate::SystemInfo;

pub struct Display {
    pub show_logo: bool,
    pub show_colors: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            show_logo: true,
            show_colors: true,
        }
    }

    pub fn print_system_info(&self, info: &SystemInfo) {
        if self.show_logo {
            self.print_side_by_side_info(info);
        } else {
            self.print_info_section(info);
        }
    }

    fn print_side_by_side_info(&self, info: &SystemInfo) {
        let logo_lines = self.print_logo();
        let info_lines = self.get_info_lines(info);
        
        let max_lines = logo_lines.len().max(info_lines.len());
        
        println!(); // Empty line at the start
        
        for i in 0..max_lines {
            let logo_line = logo_lines.get(i).map(String::as_str).unwrap_or("");
            let info_line = info_lines.get(i).map(String::as_str).unwrap_or("");
            
            // Print logo on the left (40 chars width) with extra spacing and info on the right
            if self.show_colors {
                print!("{:<40}   ", logo_line.bright_cyan());
            } else {
                print!("{:<40}   ", logo_line);
            }
            println!("{}", info_line);
        }
        
        println!(); // Empty line at the end
    }

    fn get_info_lines(&self, info: &SystemInfo) -> Vec<String> {
        let mut lines = Vec::new();
        
        lines.push(self.format_info_line("User", &info.username, Color::Green));
        lines.push(self.format_info_line("Hostname", &info.hostname, Color::Yellow));
        lines.push(self.format_info_line("OS", &info.os_name, Color::Blue));
        lines.push(self.format_info_line("Kernel", &info.kernel_version, Color::Magenta));
        lines.push(self.format_info_line("Uptime", &info.format_uptime(), Color::Cyan));
        lines.push(self.format_info_line("Shell", &info.shell, Color::Red));
        lines.push(self.format_info_line("Terminal", &info.terminal, Color::White));
        lines.push(self.format_info_line("Desktop Environment", &info.desktop_environment, Color::BrightBlue));
        lines.push(self.format_info_line("CPU", &info.cpu_info, Color::BrightGreen));
        lines.push(self.format_info_line("GPU", &info.gpu_info, Color::BrightRed));
        lines.push(self.format_info_line("Memory", &info.format_memory(), Color::BrightYellow));
        
        lines
    }

    fn format_info_line(&self, label: &str, value: &str, color: Color) -> String {
        let formatted_label = format!("{:>20}: ", label);
        
        if self.show_colors {
            format!("{}{}", formatted_label.bold(), value.color(color))
        } else {
            format!("{}{}", formatted_label, value)
        }
    }

    fn print_logo(&self) -> Vec<String> {
        let logo_lines = vec![
            "              a8888b.".to_string(),
            "             d888888b.".to_string(),
            "             8P\"YP\"Y88".to_string(),
            "             8|o||o|88".to_string(),
            "             8'    .88".to_string(),
            "             8`._.' Y8.".to_string(),
            "            d/      `8b.".to_string(),
            "          .dP   .     Y8b.".to_string(),
            "         d8:'   \"   `::88b.".to_string(),
            "        d8\"           `Y88b".to_string(),
            "       :8P     '       :888".to_string(),
            "        8a.    :      _a88P".to_string(),
            "      ._/\"Yaa_ :    .| 88P|".to_string(),
            "      \\    YP\"      `| 8P  `.".to_string(),
            "      /     \\._____.d|    .'".to_string(),
            "      `--..__)888888P`._.'".to_string(),
        ];

        if self.show_colors {
            logo_lines.iter().map(|line| line.bright_cyan().to_string()).collect()
        } else {
            logo_lines
        }
    }

    fn print_info_section(&self, info: &SystemInfo) {
        println!();
        
        self.print_info_line("User", &info.username, Color::Green);
        self.print_info_line("Hostname", &info.hostname, Color::Yellow);
        self.print_info_line("OS", &info.os_name, Color::Blue);
        self.print_info_line("Kernel", &info.kernel_version, Color::Magenta);
        self.print_info_line("Uptime", &info.format_uptime(), Color::Cyan);
        self.print_info_line("Shell", &info.shell, Color::Red);
        self.print_info_line("Terminal", &info.terminal, Color::White);
        self.print_info_line("Desktop Environment", &info.desktop_environment, Color::BrightBlue);
        self.print_info_line("CPU", &info.cpu_info, Color::BrightGreen);
        self.print_info_line("GPU", &info.gpu_info, Color::BrightRed);
        self.print_info_line("Memory", &info.format_memory(), Color::BrightYellow);
        
        println!();
    }

    fn print_info_line(&self, label: &str, value: &str, color: Color) {
        let formatted_label = if !label.is_empty() {
            format!("{:>20}: ", label)
        } else {
            format!("{:>20}: ", "")
        };

        if self.show_colors {
            println!("{}{}", formatted_label.bold(), value.color(color));
        } else {
            println!("{}{}", formatted_label, value);
        }
    }

    pub fn print_minimal(&self, info: &SystemInfo) {
        println!("{}@{}", info.username, info.hostname);
        println!("OS: {}", info.os_name);
        println!("Kernel: {}", info.kernel_version);
        println!("Uptime: {}", info.format_uptime());
        println!("CPU: {}", info.cpu_info);
        println!("GPU: {}", info.gpu_info);
        println!("Memory: {}", info.format_memory());
    }

    pub fn print_json(&self, info: &SystemInfo) -> Result<(), serde_json::Error> {
        let json_output = serde_json::json!({
            "username": info.username,
            "hostname": info.hostname,
            "os_name": info.os_name,
            "kernel_version": info.kernel_version,
            "uptime": info.uptime,
            "uptime_formatted": info.format_uptime(),
            "shell": info.shell,
            "terminal": info.terminal,
            "desktop_environment": info.desktop_environment,
            "cpu_info": info.cpu_info,
            "gpu_info": info.gpu_info,
            "memory": {
                "total": info.memory_info.total,
                "used": info.memory_info.used,
                "available": info.memory_info.available,
                "formatted": info.format_memory()
            }
        });

        println!("{}", serde_json::to_string_pretty(&json_output)?);
        Ok(())
    }
}
