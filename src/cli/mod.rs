mod debug;
mod help;
mod optional;
mod positional;

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::Command;
use std::env;

// re-export the help function
pub use help::help_wanted;

/// Initialises the Clap CLI command and sets up arguments
pub fn cli_init() -> Command {
    Command::new("mesh2ww")
        .about("Conversion of meshtal file meshes to MCNP weight windows")
        .arg_required_else_help(true)
        .disable_help_flag(true)
        .after_help(help::after_help_message())
        .long_about(help::cli_long_help())
        .term_width(76)
        .hide_possible_values(true)
        .override_usage(help::usage_message())
        .args(positional::positional_args())
        .args(optional::optional_args())
        .args(debug::debug_args())
        .styles(custom_style())
}

/// Customise the colour styles for clap v4
fn custom_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Cyan.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Magenta.on_default())
}

/// Checks if a flag is present in any of the arguments
pub fn is_flag_present(names: &[&str]) -> bool {
    env::args().any(|a| names.contains(&a.as_str()))
}
