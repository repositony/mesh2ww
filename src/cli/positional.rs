use clap::{value_parser, Arg, ArgAction};

pub fn positional_args() -> [Arg; 2] {
    [arg_meshtal(), arg_number()]
}

fn arg_meshtal() -> Arg {
    Arg::new("meshtal")
        .help_heading("Arguments")
        .help("Path to meshtal file")
        .action(ArgAction::Set)
        .value_parser(value_parser!(String))
}

fn arg_number() -> Arg {
    Arg::new("number")
        .help_heading("Arguments")
        .help("Mesh tally identifier")
        .long_help("e.g. 104 for the FMESH104:n card")
        .value_parser(value_parser!(u32))
        .action(ArgAction::Set)
}
