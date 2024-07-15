# mesh2ww

Command line tool to convert MCNP mesh tallies of any type to a mesh-based
global weight window using the magic method with configurable de-tuning options.

```text
Conversion of meshtal file meshes to MCNP weight windows

Usage: mesh2ww <meshtal> <number> [options] [+]

Arguments:
  [meshtal]  Path to meshtal file
  [number]   Mesh tally identifier

Weight options:
  -p, --power <num>...  Set the softening/de-tuning factor
  -e, --error <num>...  Maximum rel. error, use analogue above
  -t, --total           Weights from 'Total' groups only
  -s, --scale <num>     Multiply all weights by a constant
  -u, --update <path>   Path to previous weights ('cached_weights' default)

Global file options:
  -o, --output <path>  Name of output file ('wwinp' default)
      --trim           Exclude unused particles from wwinp header

Global VTK options:
      --vtk               Write VTK files for plotting
  -f, --format <fmt>      Set the VTK file format
  -r, --resolution <cst>  Cylindrical mesh resolution
      --endian <end>      Byte ordering/endian
      --compressor <cmp>  Compression method for XML

Flags:
  -v, --verbose...  Verbose logging (-v, -vv)
  -q, --quiet       Supress all log output (overrules --verbose)
      --help        Print help info (see more with '--help')

See --help for detail and examples
```

Help is printed with the `-h` flag, and `--help` will show default values,
examples, and any important behaviour.

## Install

Direct from github:

```shell
cargo install --git https://github.com/repositony/mesh2ww.git
```

All executables are under `~/.cargo/bin/`, which should already be in your path
after installing Rust.

<details>
  <summary>Click here if you have never used Rust</summary>

If you have never used the Rust programming language, the toolchain is easily
installed from the [official website](https://www.rust-lang.org/tools/install)

```shell
curl https://sh.rustup.rs -sSf | sh
```

This should have added `source $HOME/.cargo/env` to the bash profile, so update
your environment with `source ~/.bashrc`.

</details>

## Overview

### Supported mesh formats

For more detail, see the `OUT` keyword for the `FMESH` card definition in
the [MCNPv6.2](https://mcnp.lanl.gov/pdf_files/TechReport_2017_LANL_LA-UR-17-29981_WernerArmstrongEtAl.pdf)
or [MCNPv6.3](https://mcnpx.lanl.gov/pdf_files/TechReport_2022_LANL_LA-UR-22-30006Rev.1_KuleszaAdamsEtAl.pdf)
user manuals.

| Output format | Supported? | Description                                         |
| ------------- | ---------- | --------------------------------------------------- |
| COL           | Yes        | Column data (MCNP default)                          |
| CF            | Yes        | Column data including voxel volume                  |
| IJ            | Yes        | 2D matrix of I (col) and J (row) data, grouped by K |
| IK            | Yes        | 2D matrix of I (col) and K (row) data, grouped by J |
| JK            | Yes        | 2D matrix of J (col) and K (row) data, grouped by I |
| CUV (UKAEA)   | Yes        | UKAEA Cell-under-Voxel column data                  |
| NONE          | N/A        | `NONE` or unknown output format                     |

Once I get my paws on MCNPv6.3 this will be extended to include the new
COLSCI, CFSCI, and XDMF/HDF5 formats.

### Supported mesh geometries

All functionality is fully supported for both rectangular and cylindrical meshes.

| Mesh geometry | Supported? | MCNP designators |
| ------------- | ---------- | ---------------- |
| Rectangular   | Yes        | rec, xyz         |
| Cylindrical   | Yes        | cyl, rzt         |
| Spherical     | No         | sph, rpt         |

Currently spherical meshes are not supported because barely anyone knows
about them, let alone uses them. They are therefore a low priority, but raise
an issue if anyone needs it.

## Examples

### Tuning weights

Typical usage will generally define a de-tuning factor (`-p`/`--power`) and
possibly a relative error cutoff (`-e`/`--error`) for generating weights.

```bash
# Chenge the de-tuning factor and relative error cut
mesh2ww /path/to/meshtal.msht 104 --power 0.70 --error 0.25
```

The `--power` value modifies calculated weights by `w => w^(power)`, which
helps with softening extreme values. Any voxels with errors above `--error`
(>25% in this case) continue to use analogue transport until the uncertainty
imporves.

### Renaming output files

The weight window file may be renamed as needed.

```bash
# Chenge output file name to "mywwmesh.wwinp"
mesh2ww /path/to/meshtal.msht 104 --output mywwmesh.wwinp
```

### Simplified weight window

It is often fine to simply generate a global weight window using the 'Total'
group rather than every explicit energy/time group.

```bash
# Only use the Total energy/time groups
mesh2ww /path/to/meshtal.msht 104 --total
```

This is probably the recommended use case for any finely binned groups, as
nobody should really be trying to optimise for every energy in a 175-group
mesh anyway.

### Re-scale weights

Generated weights are typically normalised to the total flux for each group.
These may be rescaled by a constant multiplier.

```bash
# Multiply all normalised weights by x2.5
mesh2ww /path/to/meshtal.msht 104 --scale 2.5
```

### Multi-particle weight windows

Multiple tallies may be combined for weight windows covering multiple
particle types. This is achieved using the `+` operator.

The usage is as simple as combining multiple argument sets with `+` as the
delimiter.

```bash
# Syntax for combining weights for multiple particle types
mesh2ww <meshtal> <number> [options] +      \
        <meshtal> <number> [options] +      \
        <meshtal> <number> [options]
```

For example:

- `NP_tallies.msht` contains neutron (`FMESH14:n`) and photon (`FMESH24:p`) tallies
- `E_tallies.msht` contains an electron (`FMESH34:e`) tally

If all of these are the same geometry, they may be combined with all the
usual options applied to each tally individually:

```bash
# Make weight windows for neutrons, photons, and electrons
mesh2ww NP_tallies.msht 14                   +      \
        NP_tallies.msht 24 -p 0.8 -e 0.15    +      \
        E_tallies.msht  34 --total                  \
```

Here the neutron tally uses defaults, the photon tally de-tunes weights, and the
electron tally only uses the total energy/time bins.

### Writing weights to VTK

A Visual Toolkit file can be generated for every weight window set using the
`--vtk` flag.

**WARNING: Cylindrical weight window plotting is a WIP**

```bash
# Write to VTK for plotting 
mesh2ww file.msht 14 --vtk
```

Of course all the usual options are available, such as increasing the
resolution of cylindrical meshes with few theta bins.

```bash
# Change cylindrical mesh resolution
mesh2ww file.msht 14 --vtk --resolution 2
```

Advanced options include changing the file format, byte ordering of binary
outputs, and which compressor to use for XML.

```bash
# Change VTK file format options
mesh2ww file.msht 14 --vtk          \
            --format legacy-ascii   \
            --compressor lzma       \
            --endian big-endian
```

### Advanced de-tuning

For fine control, the `--power` and `--error` parameters may be set
explicitly for every unique group.

For example, if a mesh has 3 energy groups at `1.0 MeV`, `10.0 MeV`, and
`100.0 MeV`, the power factor for each may be set to `0.8`, `0.7`, and `0.65`
respectively.

```bash
# Set energy group power factors individually
mesh2ww /path/to/meshtal.msht 104 --power 0.8 0.7 0.65
```

This also applies to time bins. To set values for all unique
groups, the values must be given in the order expected by MCNP.

For example, a mesh with 3x energy groups and 2x time groups:

```text
Energy 1.0        Power
    Time 1e10      0.9
    Time 1e20      0.7
Energy 10.0
    Time 1e10      0.8
    Time 1e20      0.8
Energy 100.0
    Time 1e10      0.6
    Time 1e20      0.5
```

```bash
# Set energy and time group power factors individually
mesh2ww /path/to/meshtal.msht 104 --power 0.9 0.7   0.8 0.8   0.6 0.5
```
