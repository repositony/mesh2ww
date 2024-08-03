use crate::logging;
use crate::parser::{self, VtkConfig, WWConfig};
use crate::wrappers::{CliByteOrder, CliCompressor, CliVtkFormat};

use ntools::mesh::reader::MeshtalReader;
use ntools::mesh::{Mesh, Particle};
use ntools::utils::f;
use ntools::weights::vtk::{write_vtk, WeightsToVtk, WeightsToVtkBuilder};
use ntools::weights::WeightWindow;
use ntools::wwgen;

use vtkio::model::ByteOrder;
use vtkio::xml::Compressor;

use anyhow::{anyhow, Result};
use log::{debug, info, warn};
use std::path::Path;

pub fn collect_weight_windows(ww_config_sets: Vec<WWConfig>) -> Result<Vec<WeightWindow>> {
    // prepare for writing to VTK files if needed
    let vtk_config = parser::vtk_config();

    // prepare the ultimate return value
    let mut weight_windows: Vec<WeightWindow> = Vec::with_capacity(ww_config_sets.len());

    // Process each weight window set
    for cli in &ww_config_sets {
        // read mesh data from the meshtal file
        info!("Reading mesh {} from {}", &cli.number, &cli.meshtal);
        let mesh = try_meshtal_read(cli)?;

        // make sure the particle type is not a duplicate
        if weight_windows
            .iter()
            .any(|ww| ww.particle == mesh.particle as u8)
        {
            info!("{:?} already included, skipping...", mesh.particle);
            continue;
        }

        // convert mesh into WWMesh object for writing/further manipulation
        info!("Calculating {:?} weights", &mesh.particle);
        let mut ww = generate_weight_window(&mesh, cli);

        // Multiply weights by a constant factor if one is provided
        if cli.scale != 1.0 {
            info!("Scaling results by {}", cli.scale);
            ww.scale(cli.scale);
        }

        info!(
            "{:?} voxels with non-zero weight: {:.2}%",
            Particle::from_id(ww.particle),
            ww.non_analogue_percentage()
        );

        // Write this out to a VTK for plotting is needed
        if vtk_config.vtk {
            info!("Writing {:?} VTK file", Particle::from_id(ww.particle));
            generate_vtk(&ww, &vtk_config)?;
        }

        weight_windows.push(ww);
    }

    if weight_windows.is_empty() {
        Err(anyhow!("No valid weight window sets"))
    } else {
        Ok(weight_windows)
    }
}

fn try_meshtal_read(cli: &WWConfig) -> Result<Mesh> {
    let path: &Path = Path::new(&cli.meshtal);

    let mut reader = MeshtalReader::new();
    reader.set_target_id(cli.number);
    if logging::is_quiet() || logging::verbosity() > 1 {
        reader.disable_progress();
    }

    let mut mesh = reader.parse(path)?;
    Ok(std::mem::take(&mut mesh[0]))
}

fn generate_weight_window(mesh: &Mesh, cli: &WWConfig) -> WeightWindow {
    if cli.power.len() > 1 || cli.error.len() > 1 {
        if cli.total {
            warn!("Warning: Conflicting options");
            warn!(" - Multiple --power/--error values used with --total");
            warn!(" - Falling back to default values (p=0.7, e=1.0)");
            wwgen::mesh_to_ww(mesh, 0.7, 1.0, cli.total)
        } else {
            wwgen::mesh_to_ww_advanced(mesh, &cli.power, &cli.error)
        }
    } else {
        wwgen::mesh_to_ww(mesh, cli.power[0], cli.error[0], cli.total)
    }
}

fn generate_vtk(weight_window: &WeightWindow, cli: &VtkConfig) -> Result<()> {
    // Set up the conversion
    let convertor = build_converter(cli);
    let vtk = convertor.convert(weight_window);
    let extension = match cli.format {
        CliVtkFormat::Xml => match weight_window.nwg {
            // Mesh type 1=rec, 2=cyl, 3=sph
            1 => "vtr",
            2 => "vtu",
            _ => "vtk",
        },
        _ => "vtk",
    };

    debug!(
        "Ouput file: \"{}\"",
        f!(
            "ww_{:?}.{extension}",
            Particle::from_id(weight_window.particle)
        )
        .to_lowercase()
    );

    // Write to disk, using the paticle type as a simple file name
    write_vtk(
        vtk,
        f!(
            "ww_{:?}.{extension}",
            Particle::from_id(weight_window.particle)
        )
        .to_lowercase(),
        cli.format.into(),
    )
    .map_err(|e| anyhow!(e))
}

fn build_converter(cli: &VtkConfig) -> WeightsToVtk {
    WeightsToVtkBuilder::default()
        .resolution(cli.resolution)
        .byte_order(match cli.endian {
            CliByteOrder::BigEndian => ByteOrder::BigEndian,
            CliByteOrder::LittleEndian => ByteOrder::LittleEndian,
        })
        .compressor(match cli.compressor {
            CliCompressor::LZMA => Compressor::LZMA,
            CliCompressor::LZ4 => Compressor::LZ4,
            CliCompressor::ZLib => Compressor::ZLib,
            CliCompressor::None => Compressor::None,
        })
        .build()
}
