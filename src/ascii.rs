/// ASCII art logos for various Linux distributions and operating systems
/// Based on the pfetch project by Dylan Araps
/// https://github.com/dylanaraps/pfetch

pub struct Logo {
    pub lines: Vec<&'static str>,
    pub colors: Vec<&'static str>, // ANSI color codes for each color used
}

// ANSI color codes - Basic colors (30-37)
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

// Bright colors (90-97) - only the ones we use
const BRIGHT_CYAN: &str = "\x1b[96m";

// Color aliases for specific logo schemes
const C1: &str = RED;
const C2: &str = GREEN;
const C3: &str = YELLOW;
const C4: &str = BLUE;
const C5: &str = MAGENTA;
const C6: &str = CYAN;
const C7: &str = WHITE;
const C14: &str = BRIGHT_CYAN;

pub fn get_distro_logo(distro_name: &str) -> Option<Logo> {
    let distro_lower = distro_name.to_lowercase();
    
    // Match distribution names and return their ASCII art
    match distro_lower.as_str() {
        name if name.contains("cachyos") || name.contains("cachy") => Some(Logo {
            lines: vec![
                "   .##\\#####+   o",
                " /\\##\"\\###`",
                " /',\\/        +=",
                " -==|         `` _",
                " \\###\\           ###",
                "  \\#|/#####\\##+   `",
                "  `/########`",
            ],
            colors: vec![C2, C6], // Green for main parts, cyan for accents
        }),
        
        name if name.contains("alma") => Some(Logo {
            lines: vec![
                "    {#@ .,       ,..<.",
                "    ._`=#  /#=#,\"\"",
                "   \\##  ` |`  '=##",
                "   .`'=.  \\  ,_,.,",
                " ..\"#-   `  `    \\##",
                "\\## #   ./  \\.  .#`,+",
                "   =##='  ,   |,  |# \"\"",
                "      =##=++#|",
                "         ##\\''",
                "         `'`",
            ],
            colors: vec![C1, C3, C4, C2, C6], // Red, yellow, blue, green, cyan
        }),
        
        name if name.contains("alpine") => Some(Logo {
            lines: vec![
                "   /\\ /\\",
                "  /  \\  \\",
                " /    \\  \\",
                "//      \\  \\",
                "//        \\ \\",
                "           \\",
            ],
            colors: vec![C4, C7], // Blue and white
        }),
        
        name if name.contains("arch") => Some(Logo {
            lines: vec![
                "       /\\",
                "      /  \\",
                "     /\\   \\",
                "    /      \\",
                "   /   ,,   \\",
                "  /   |  |  -\\",
                " /_-''    ''-_\\",
            ],
            colors: vec![C6, C4], // Cyan for main, blue for accent
        }),
        
        name if name.contains("ubuntu") => Some(Logo {
            lines: vec![
                "         _",
                "     ---(_)",
                " _/  ---  \\",
                "(_) |   |",
                " \\  --- _/",
                "     ---(_)",
            ],
            colors: vec![C3], // Yellow/orange
        }),
        
        name if name.contains("debian") => Some(Logo {
            lines: vec![
                "  _____",
                " /  __ \\",
                "|  /    |",
                "|  \\___-",
                "-_",
                "  --_",
            ],
            colors: vec![C1], // Red
        }),
        
        name if name.contains("fedora") => Some(Logo {
            lines: vec![
                "        ,''''.",
                "       |   ,. |",
                "       |  |  '_'",
                "  ,....|  |..",
                ".'  ,_;|   ..'",
                "|  |   |  |",
                "|  ',_,'  |",
                " '.     ,'",
                "   '''''",
            ],
            colors: vec![C4], // Blue
        }),
        
        name if name.contains("gentoo") => Some(Logo {
            lines: vec![
                " _-----_",
                "(       \\",
                "\\    0   \\",
                " \\        )",
                " /      _/",
                "(     _-",
                "\\____-",
            ],
            colors: vec![C5, C7], // Magenta and white
        }),
        
        name if name.contains("manjaro") => Some(Logo {
            lines: vec![
                "||||||||| ||||",
                "||||||||| ||||",
                "||||      ||||",
                "|||| |||| ||||",
                "|||| |||| ||||",
                "|||| |||| ||||",
                "|||| |||| ||||",
            ],
            colors: vec![C2], // Green
        }),
        
        name if name.contains("mint") || name.contains("linuxmint") => Some(Logo {
            lines: vec![
                " ___________",
                "|_          \\",
                "  | | _____ |",
                "  | | | | | |",
                "  | | | | | |",
                "  | \\__\\_/ |",
                " \\_________/",
            ],
            colors: vec![C2, C7], // Green and white
        }),
        
        name if name.contains("opensuse") || name.contains("suse") => Some(Logo {
            lines: vec![
                "  _______",
                "__|   __ \\",
                "     / .\\ \\",
                "     \\__/ |",
                "   _______|",
                "   \\_______",
                "__________/",
            ],
            colors: vec![C2], // Green
        }),
        
        name if name.contains("centos") => Some(Logo {
            lines: vec![
                " ____^____",
                " |\\  |  /|",
                " | \\ | / |",
                "<---- ---->",
                " | / | \\ |",
                " |/__| __\\|",
                "     v",
            ],
            colors: vec![C2, C3, C5, C4], // Green, yellow, magenta, blue
        }),
        
        name if name.contains("void") => Some(Logo {
            lines: vec![
                "    _______",
                " _ \\______ -",
                "| \\  ___  \\ |",
                "| | /   \\ | |",
                "| | \\___/ | |",
                "| \\______ \\_|",
                " -_______\\",
            ],
            colors: vec![C2], // Green
        }),
        
        name if name.contains("pop") => Some(Logo {
            lines: vec![
                "    .///////,",
                "   //76767//////",
                " //76//76//767//",
                " ////7676'//76////",
                "////76////7/////",
                "  ////76//76////",
                "  //76767676//",
                "     `///////'",
            ],
            colors: vec![C6, C7], // Cyan and white
        }),
        
        name if name.contains("endeavour") || name.contains("endeavouros") => Some(Logo {
            lines: vec![
                "      /\\",
                "    /  \\",
                "   /  /\\  \\",
                "  /  /   \\ \\",
                " /  /     _) )",
                "/_/___-- __-",
                " /____--",
            ],
            colors: vec![C1, C5, C4], // Red, magenta, blue
        }),
        
        name if name.contains("garuda") => Some(Logo {
            lines: vec![
                "         _______",
                "     __/       \\_",
                "    _/     /      \\_",
                "  _/     /_________\\",
                "_/                  |",
                "\\     ____________",
                " \\_           __/",
                "   \\__________/",
            ],
            colors: vec![C3, C7, C2], // Yellow, white, green
        }),
        
        name if name.contains("nixos") || name.contains("nix") => Some(Logo {
            lines: vec![
                "  \\\\  \\\\ //",
                " ==\\\\__\\\\/ //",
                "   //   \\\\//",
                "==//     //==",
                " //\\\\___//",
                "// /\\\\ \\\\==",
                "  // \\\\  \\\\",
            ],
            colors: vec![C4], // Blue
        }),
        
        name if name.contains("freebsd") => Some(Logo {
            lines: vec![
                "/\\,-'''''-,/\\",
                "\\_)       (_/",
                "|           |",
                "|           |",
                " ;         ;",
                "  '-_____-'",
            ],
            colors: vec![C1], // Red
        }),
        
        name if name.contains("openbsd") => Some(Logo {
            lines: vec![
                "      _____",
                "    \\-     -/",
                " \\_/         \\",
                " |    O   O   |",
                " |_  <   )  3 )",
                " / \\         /",
                "    /-_____-\\",
            ],
            colors: vec![C3, C7], // Yellow and white
        }),
        
        name if name.contains("macos") || name.contains("darwin") => Some(Logo {
            lines: vec![
                "      .:'",
                "    _ :'_",
                " .'`_`-'_``.",
                ":________.-'",
                ":_______:",
                " :_______`-;",
                "  `._.-._.'",
            ],
            colors: vec![C2, C6, C3, C1, C5], // Green, cyan, yellow, red, magenta
        }),
        
        name if name.contains("android") => Some(Logo {
            lines: vec![
                "  ;,           ,;",
                "   ';,.-----.,;'",
                "  ,'           ',",
                " /    O     O    \\",
                "|                |",
                "'-----------------'",
            ],
            colors: vec![C2], // Green
        }),
        
        name if name.contains("linux") => Some(Logo {
            lines: vec![
                "    ___",
                "   (.. |",
                "   (<> |",
                "  /  __  \\",
                " ( /  \\ /|",
                "_/\\ __)/_)",
                "\\/-____\\/",
            ],
            colors: vec![C4, C5, C7], // Blue, magenta, white
        }),
        
        name if name.contains("elementary") => Some(Logo {
            lines: vec![
                "  _______",
                " / ____  \\",
                "/  |  / /\\",
                "|__\\ /  / |",
                "\\   /__/  /",
                " \\_______/",
            ],
            colors: vec![C7], // White
        }),
        
        name if name.contains("zorin") => Some(Logo {
            lines: vec![
                "      \\\\\\\\\\\\\\\\",
                "     \\\\\\\\\\\\\\\\\\\\",
                "    \\\\\\\\\\\\\\\\\\\\\\\\",
                "   ////////////////",
                "  ////////////////",
                " ////////////////",
            ],
            colors: vec![C4], // Blue
        }),
        
        name if name.contains("kali") => Some(Logo {
            lines: vec![
                "      ________",
                "     /  ______\\",
                "    / /        \\",
                "   | |    _____/",
                "   | |   |__",
                "   | |      __|",
                "    \\ \\____/  /",
                "     \\_______/",
            ],
            colors: vec![C4], // Blue
        }),
        
        name if name.contains("artix") => Some(Logo {
            lines: vec![
                "      /\\",
                "     /  \\",
                "    /`'.,\\",
                "   /     ',",
                "  /      ,`\\",
                " / ,.'. ` \\",
                "/.,`     `.\\",
            ],
            colors: vec![C6], // Cyan
        }),
        
        name if name.contains("solus") => Some(Logo {
            lines: vec![
                "",
                "     /|",
                "    / | \\",
                "   /  |  \\ _",
                "  /___|__\\_\\",
                " \\         /",
                "  `-------´",
            ],
            colors: vec![C6], // Cyan
        }),
        
        _ => None,
    }
}

pub fn get_default_logo() -> Logo {
    Logo {
        lines: vec![
            "              a8888b.",
            "             d888888b.",
            "             8P\"YP\"Y88",
            "             8|o||o|88",
            "             8'    .88",
            "             8`._.' Y8.",
            "            d/      `8b.",
            "          .dP   .     Y8b.",
            "         d8:'   \"   `::88b.",
            "        d8\"           `Y88b",
            "       :8P     '       :888",
            "        8a.    :      _a88P",
            "      ._/\"Yaa_ :    .| 88P|",
            "      \\    YP\"      `| 8P  `.",
            "      /     \\._____.d|    .'",
            "      `--..__)888888P`._.'",
        ],
        colors: vec![C14], // Bright cyan
    }
}
