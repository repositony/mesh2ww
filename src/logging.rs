use crate::cli::is_flag_present;

use anyhow::Result;
use std::env;

/// Sets up logging at runtime to allow for multiple verbosity levels
pub fn init_logging() -> Result<()> {
    let verbosity = verbosity();
    let show_level = verbosity > 0;

    Ok(stderrlog::new()
        .quiet(is_quiet())
        .verbosity(verbosity + 2)
        .show_level(show_level)
        .color(stderrlog::ColorChoice::Auto)
        .timestamp(stderrlog::Timestamp::Off)
        .init()?)
    // .module(module_path!())
}

/// Gets the total requested verbosity
pub fn verbosity() -> usize {
    env::args()
        .filter(|p| (p.starts_with("-v") || p.as_str() == "--verbose"))
        .fold(0, |total, arg| match arg.as_str() {
            "--verbose" => total + 1,
            _ => total + arg.matches('v').count(),
        })
}

/// Check for quiet flag
pub fn is_quiet() -> bool {
    is_flag_present(&["-q", "--quiet"])
}
