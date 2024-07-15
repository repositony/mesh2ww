use clap::{Arg, ArgAction};

pub fn debug_args() -> [Arg; 3] {
    [arg_verbosity(), arg_quiet(), arg_help()]
}

fn arg_verbosity() -> Arg {
    Arg::new("verbose")
        .short('v')
        .long("verbose")
        .help_heading("Flags")
        .help("Verbose logging (-v, -vv)")
        .long_help(
            "If specified, the default log level of INFO is increased to DEBUG (-v) or TRACE (-vv). Errors and Warnings are always logged unless quiet (-q) is used.",
        )
        .required(false)
        .action(ArgAction::Count)
}

fn arg_quiet() -> Arg {
    Arg::new("quiet")
        .short('q')
        .long("quiet")
        .help_heading("Flags")
        .help("Supress all log output (overrules --verbose)")
        .required(false)
        .action(ArgAction::SetTrue)
}

fn arg_help() -> Arg {
    Arg::new("help")
        .long("help")
        .help_heading("Flags")
        .help("Print help info (see more with '--help')")
        .required(false)
        .action(ArgAction::HelpShort)
}
