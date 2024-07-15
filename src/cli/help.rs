use crate::cli::{cli_init, is_flag_present};

pub fn help_wanted() -> bool {
    if is_flag_present(&["--help"]) {
        let mut command = cli_init();
        command
            .print_long_help()
            .expect("Could not print help message");
        true
    } else if is_flag_present(&["-h"]) {
        let mut command = cli_init();
        command.print_help().expect("Could not print help message");
        true
    } else {
        false
    }
}

pub fn usage_message() -> &'static str {
    "mesh2ww <meshtal> <number> [options] [+]"
}

pub fn after_help_message() -> &'static str {
    "See --help for detail and examples"
}

/// Full help message
pub fn cli_long_help() -> &'static str {
    "Conversion of meshtal file meshes to MCNP weight windows
    
For multiple particle types, use the '+' operator to combine multiple tallies that have the same dimensions.

Use the --vtk flag to generate Visual Toolkit files for plotting.

For advanced users, the --power and --error de-tuning factors may be set for individual energy/time groups. All groups must be explicitly provided.

Supports all mesh output formats for rectangular and cylindrical geometries. 

Typical examples 
----------------

    Convert single tally with defaults  
        $ mesh2ww file.msht 14

    Change the softening/de-tuning factor  
        $ mesh2ww file.msht 14 --power 0.8 

    Only generate weights for voxels with <10% error
        $ mesh2ww file.msht 14 --error 0.1

    Only use the 'Total' energy/time groups 
        $ mesh2ww file.msht 14 --total

    Multiply all weights by a constant factor
        $ mesh2ww file.msht 14 --scale 2.0


Mutli-particle examples 
-----------------------

    Use the '+' operator to combine meshes (same dimensions):
        $ mesh2ww file.msht 14 + run0.msht 24

    All options can be applied individually:
        $ mesh2ww fileA 14 -p 0.8 --scale 10    \\
                + fileB 24 -p 0.5 -e 0.15       \\
                + fileC 14 --total 

VTK plotting outputs 
--------------------

    Output a vtk for all weight window sets:
        $ mesh2ww file.msht 14 --vtk

    Make cylindrical meshes look rounder:
        $ mesh2ww file.msht 14 --vtk --resolution 2

    Change other advanced fromatting options:
        $ mesh2ww file.msht 14 --vtk    \\
                --format legacy-ascii   \\
                --compressor lzma       \\ 
                --endian big-endian     

Advanced de-tuning
------------------
    
    Set power factors individually for a 3x erg group mesh
        $ mesh2ww file.msht 104 --power 0.8 0.7 0.65

    Set both factors individually for a 3x erg group mesh
        $ mesh2ww file.msht 104         \\
                  --power 0.8 0.7 0.65  \\
                  --error 1.0 0.9 1.0

    Set factors individually for 3x erg group, 2x time groups
        $ mesh2ww file.msht 104    \\
                  --power 0.8 0.7  \\   => (e0,t0) (e0,t1)
                          0.9 0.8  \\   => (e1,t0) (e1,t1)
                          0.7 0.6  \\   => (e2,t0) (e2,t1)
                  
Notes
-----

The MAGIC method is used to convert tallies to mesh-based global weight windows. Weights are calculated as (0.5 * norm_flux)^power. Any voxels with errors larger than --error are set to analogue. Flux data are normalised by the maximum flux of each energy/time group.

CuV voidoff=yes will not output results for void cells. These will therefore always be analogue. CuV also has a habit of including -ve results, which are unphysical and considered to be 0.0 in this implementation."
}
