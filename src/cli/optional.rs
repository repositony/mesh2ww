use crate::wrappers::{CliByteOrder, CliCompressor, CliVtkFormat};
use clap::{value_parser, Arg, ArgAction};

pub fn optional_args() -> [Arg; 12] {
    [
        arg_power(),
        arg_error(),
        arg_total(),
        arg_scale(),
        arg_output(),
        arg_padding(),
        arg_vtk(),
        arg_format(),
        arg_resolution(),
        arg_endian(),
        arg_compressor(),
        // arg_update(),
    ]
}

fn arg_power() -> Arg {
    Arg::new("power")
            .short('p')
            .long("power")
            .help_heading("Weight options")
            .help("Set the softening/de-tuning factor")
            .long_help(
                "Set the softening/de-tuning factor\n\nDefault 0.70. The softening/de-tuning factor is applied to the weights as ww => ww^(<num>).\n\nFor advanced use, multiple values are provided. These will apply to each energy/time group individually (see examples above).",
            )
            .required(false)
            .action(ArgAction::Set)
            .value_delimiter(' ')
            .num_args(1..)
            .value_parser(value_parser!(f64))
            .default_value("0.7")
            .value_name("num")
            .hide_default_value(true)
}

fn arg_error() -> Arg {
    Arg::new("error")
            .short('e')
            .long("error")
            .help_heading("Weight options")
            .help("Maximum rel. error, use analogue above")
            .long_help(
                "Maximum rel. error, use analogue above\n\nDefault 1.0 (100%). Relative errors above the provided value are set to zero, and will continue to use analogue transport until better statistics are available.\n\nFor advanced use, multiple values are provided. These will apply to each energy/time group individually (see examples above).",
            )
            .required(false)
            .action(ArgAction::Set)
            .value_delimiter(' ')
            .num_args(1..)
            .value_parser(value_parser!(f64))
            .default_value("1.0")
            .value_name("num")
            .hide_default_value(true)
}

fn arg_total() -> Arg {
    Arg::new("total")
            .short('t')
            .long("total")
            .help_heading("Weight options")
            .help("Weights from 'Total' groups only")
            .long_help(
                "Weights from 'Total' groups only\n\nOften it can be desirable to simply generate the weight window mesh from the 'Total' groups rather than every explicit energy/time group.\n\nThis probably the recommended use case for any finely binned groups, as nobody should really be trying to optimise for every energy in a 175-group mesh anyway.",
            )
            .required(false)
            .action(ArgAction::SetTrue)
}

fn arg_scale() -> Arg {
    Arg::new("scale")
            .short('s')
            .long("scale")
            .help_heading("Weight options")
            .help("Multiply all weights by a constant")
            .long_help(
                "Multiply all weights by a constant\n\nAll weights calculated from the mesh are typically normalised to the total flux. These may be rescaled by the value provided. e.g. --scale 10 will multiply every weight by 10.0",
            )
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(f64))
            .default_value("1.0")
            .value_name("num")
            .hide_default_value(true)
}

fn arg_output() -> Arg {
    Arg::new("output")
        .short('o')
        .long("output")
        .help_heading("Global file options")
        .help("Name of output file ('wwinp' default)")
        .long_help(
            "Defaults to \"wwinp\". Ouptut formatted to WWOUT file specification from the MCNP user manuals.",
        )
        .required(false)
        .action(ArgAction::Set)
        .value_parser(value_parser!(String))
        .value_name("path")
        .hide_default_value(true)
}

fn arg_padding() -> Arg {
    Arg::new("trim")
        .long("trim")
        .help_heading("Global file options")
        .help("Exclude unused particles from wwinp header")
        .long_help("For multiple particle types, it is unclear (without MCNP source access) how the header is read. Experience says you need to pad the header with zeros for all the unused particle types, ordered by particle id. If this is not the case, then --trim exists to get rid of the padding.")
        .required(false)
        .action(ArgAction::SetTrue)
}

fn arg_vtk() -> Arg {
    Arg::new("vtk")
        .long("vtk")
        .help_heading("Global VTK options")
        .help("Write VTK files for plotting")
        .long_help("Flag to specify that visual toolkit plot formats should be generated for each weight window set.")
        .required(false)
        .action(ArgAction::SetTrue)
}

fn arg_resolution() -> Arg {
    Arg::new("resolution")
        .short('r')
        .long("resolution")
        .help_heading("Global VTK options")
        .help("Cylindrical mesh resolution")
        .long_help(
            "WARNING: Every vertex is defined explicitly, so large values will significantly increase memory usage and file size.\n\nInteger value for increasing angular resolution of cylindrical meshes. Cylinders are approximated to straight edge segments so it can be useful to round this off by splitting voxels into multiple smaller segments.\n\ne.g. 4 theta bins gives 4 edges and therefore looks square. Using `--resolution 3` generates 12 edges instead and looks more rounded.",
        )
        .required(false)
        .action(ArgAction::Set)
        .value_parser(value_parser!(u8))
        .value_name("cst")
        .hide_default_value(true)
}

fn arg_format() -> Arg {
    Arg::new("format")
        .short('f')
        .long("format")
        .help_heading("Global VTK options")
        .help("Set the VTK file format")
        .long_help(
            "Available visual toolkit file formats:
    > xml (default)
    > legacy-ascii
    > legacy-binary",
        )
        .required(false)
        .action(ArgAction::Set)
        .value_parser(value_parser!(CliVtkFormat))
        .value_name("fmt")
        .hide_default_value(true)
}

fn arg_endian() -> Arg {
    Arg::new("endian")
        .long("endian")
        .help_heading("Global VTK options")
        .help("Byte ordering/endian")
        .long_help(
            "Visit only reads big endian, most sytems are little endian. Defaults to big endian for convenience.
    > big-endian (default)
    > little-endian",
        )
        .required(false)
        .action(ArgAction::Set)
        .value_parser(value_parser!(CliByteOrder))
        .value_name("end")
        .hide_default_value(true)
}

fn arg_compressor() -> Arg {
    Arg::new("compressor")
        .long("compressor")
        .help_heading("Global VTK options")
        .help("Compression method for XML")
        .long_help(
            "Generally just use LZMA but other options are available.
    > lzma (default)
    > lz4
    > zlib
    > none",
        )
        .required(false)
        .action(ArgAction::Set)
        .value_parser(value_parser!(CliCompressor))
        .value_name("cmp")
        .hide_default_value(true)
}

// fn arg_update() -> Arg {
//     Arg::new("update")
//         .short('u')
//         .long("update")
//         .help_heading("Weight options")
//         .help("Path to previous weights ('cached_weights' default)")
//         .long_help(
//             "Defaults to \"cached_weights\". Contains weights and associated errors for updating better weight values in iterative runs.",
//         )
//         .required(false)
//         .action(ArgAction::Set)
//         .value_parser(value_parser!(String))
//         .value_name("path")
//         .hide_default_value(true)
// }
