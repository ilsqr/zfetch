use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub minimal: bool,
    pub no_color: bool,
    pub no_logo: bool,
    pub json_output: bool,
    pub help: bool,
    pub version: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            minimal: false,
            no_color: false,
            no_logo: false,
            json_output: false,
            help: false,
            version: false,
        }
    }

    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut config = Self::new();

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-m" | "--minimal" => config.minimal = true,
                "-n" | "--no-color" => config.no_color = true,
                "-l" | "--no-logo" => config.no_logo = true,
                "-j" | "--json" => config.json_output = true,
                "-h" | "--help" => config.help = true,
                "-v" | "--version" => config.version = true,
                _ => {
                    eprintln!("Unknown argument: {}", arg);
                    config.help = true;
                }
            }
        }

        config
    }

    pub fn print_help() {
        println!("zfetch - Yet another system information fetcher for Linux");
        println!();
        println!("USAGE:");
        println!("    zfetch [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -m, --minimal     Display minimal information");
        println!("    -n, --no-color    Disable colored output");
        println!("    -l, --no-logo     Don't display the logo");
        println!("    -j, --json        Output information in JSON format");
        println!("    -h, --help        Show this help message");
        println!("    -v, --version     Show version information");
        println!();
        println!("EXAMPLES:");
        println!("    zfetch              # Show full system information");
        println!("    zfetch --minimal    # Show minimal information");
        println!("    zfetch --json       # Output as JSON");
        println!("    zfetch --no-color   # Disable colors");
    }

    pub fn print_version() {
        println!("zfetch v{}", env!("CARGO_PKG_VERSION"));
        println!("Yet another system information fetcher for Linux");
        println!("Written in Rust for performance and reliability");
    }
}
