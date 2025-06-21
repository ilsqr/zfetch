use crate::{SystemInfo, ascii};

pub struct Display {
    pub show_logo: bool,
    pub show_colors: bool,
}

// ANSI renk kodları - terminalin kendi renk şemasını kullanır
const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

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
        
        // Calculate logo width (without ANSI codes for proper alignment)
        let logo_width = logo_lines.iter()
            .map(|line| self.strip_ansi_codes(line).len())
            .max()
            .unwrap_or(0);
        
        println!(); // Empty line at the start
        
        for i in 0..max_lines {
            let logo_line = logo_lines.get(i).map(String::as_str).unwrap_or("");
            let info_line = info_lines.get(i).map(String::as_str).unwrap_or("");
            
            // Calculate padding to align logo properly
            let logo_display_width = self.strip_ansi_codes(logo_line).len();
            let padding = logo_width.saturating_sub(logo_display_width);
            
            // Print logo with consistent spacing and info on the right
            print!("{}{} ", logo_line, " ".repeat(padding + 2));
            println!("{}", info_line);
        }
        
        println!(); // Empty line at the end
    }

    fn get_info_lines(&self, info: &SystemInfo) -> Vec<String> {
        vec![
            self.format_info_line("User", &info.username, WHITE),
            self.format_info_line("Hostname", &info.hostname, WHITE),
            self.format_info_line("OS", &info.os_name, WHITE),
            self.format_info_line("Kernel", &info.kernel_version, WHITE),
            self.format_info_line("Uptime", &info.format_uptime(), WHITE),
            self.format_info_line("Shell", &info.shell, WHITE),
            self.format_info_line("Terminal", &info.terminal, WHITE),
            self.format_info_line("Desktop Environment", &info.desktop_environment, WHITE),
            self.format_info_line("CPU", &info.cpu_info, WHITE),
            self.format_info_line("GPU", &info.gpu_info, WHITE),
            self.format_info_line("Memory", &info.format_memory(), WHITE),
        ]
    }

    fn format_info_line(&self, label: &str, value: &str, color: &str) -> String {
        let formatted_label = format!("{:>20}: ", label);
        
        if self.show_colors {
            format!("{}{}{}{}{}{}", CYAN, formatted_label, RESET, color, value, RESET)
        } else {
            format!("{}{}", formatted_label, value)
        }
    }

    fn strip_ansi_codes(&self, text: &str) -> String {
        // Simple ANSI code removal - removes sequences like \x1b[31m, \x1b[0m etc.
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Skip the escape sequence
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume '['
                    // Skip until we find a letter (end of ANSI sequence)
                    while let Some(c) = chars.next() {
                        if c.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            } else {
                result.push(ch);
            }
        }
        result
    }

    fn print_logo(&self) -> Vec<String> {
        // Try to get distribution-specific logo first
        let logo = if let Some(distro_logo) = ascii::get_distro_logo(&self.get_os_name()) {
            distro_logo
        } else {
            // Fallback to default logo
            ascii::get_default_logo()
        };

        // Apply colors to logo lines
        if self.show_colors && !logo.colors.is_empty() {
            logo.lines.iter().enumerate().map(|(i, line)| {
                // Use different colors for different parts of the logo
                let color_index = match logo.colors.len() {
                    1 => 0, // Single color
                    2 => i % 2, // Alternate between two colors
                    3 => match i {
                        0..=2 => 0,
                        3..=5 => 1,
                        _ => 2,
                    },
                    _ => i % logo.colors.len(), // Cycle through all colors
                };
                let color = logo.colors.get(color_index).unwrap_or(&logo.colors[0]);
                format!("{}{}{}", color, line, "\x1b[0m")
            }).collect()
        } else {
            logo.lines.iter().map(|s| s.to_string()).collect()
        }
    }

    fn get_os_name(&self) -> String {
        // This is a simple way to get OS name for logo selection
        std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|contents| {
                contents.lines()
                    .find(|line| line.starts_with("ID="))
                    .map(|line| {
                        let id = line.strip_prefix("ID=").unwrap_or("linux");
                        // Remove quotes if present
                        id.trim_matches('"').to_string()
                    })
            })
            .unwrap_or_else(|| "linux".to_string())
    }

    fn print_info_section(&self, info: &SystemInfo) {
        println!();
        
        let info_data = [
            ("User", &info.username, WHITE),
            ("Hostname", &info.hostname, WHITE),
            ("OS", &info.os_name, WHITE),
            ("Kernel", &info.kernel_version, WHITE),
            ("Uptime", &info.format_uptime(), WHITE),
            ("Shell", &info.shell, WHITE),
            ("Terminal", &info.terminal, WHITE),
            ("Desktop Environment", &info.desktop_environment, WHITE),
            ("CPU", &info.cpu_info, WHITE),
            ("GPU", &info.gpu_info, WHITE),
            ("Memory", &info.format_memory(), WHITE),
        ];

        for (label, value, color) in info_data {
            self.print_info_line(label, value, color);
        }
        
        println!();
    }

    fn print_info_line(&self, label: &str, value: &str, color: &str) {
        let formatted_label = format!("{:>20}: ", label);

        if self.show_colors {
            println!("{}{}{}{}{}{}", CYAN, formatted_label, RESET, color, value, RESET);
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
