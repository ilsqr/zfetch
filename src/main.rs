use zfetch::{SystemInfo, Display, Config};

fn main() {
    let config = Config::from_args();

    if config.help {
        Config::print_help();
        return;
    }

    if config.version {
        Config::print_version();
        return;
    }

    // Gather system information
    let system_info = SystemInfo::new();

    // Configure display
    let mut display = Display::new();
    display.show_colors = !config.no_color;
    display.show_logo = !config.no_logo;

    // Display the information based on configuration
    if config.json_output {
        if let Err(e) = display.print_json(&system_info) {
            eprintln!("Error outputting JSON: {}", e);
            std::process::exit(1);
        }
    } else if config.minimal {
        display.print_minimal(&system_info);
    } else {
        display.print_system_info(&system_info);
    }
}
