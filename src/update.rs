// standard library
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

// external crates
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CachedWeights {
    pub particle: u8,
    pub weight_error: Vec<(f64, f64)>,
}

/// Generate a binary file for the cached weights
pub fn read_binary(path: impl AsRef<Path>) -> Result<CachedWeights> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(bincode::deserialize_from(reader)?)
}

/// Generate a binary file for the cached weights
pub fn write_binary(path: impl AsRef<Path>, cached: &CachedWeights) -> Result<()> {
    let f = BufWriter::new(File::create(path)?);
    Ok(bincode::serialize_into(f, cached)?)
}

// pub fn prefetch_binary<P: AsRef<Path>>(path: P, rad_type: RadType) -> Result<()> {
//     let weight_error = zip(&weight_window.weights, &mesh.voxels)
//         .map(|(w, v)| (*w, v.error))
//         .collect();

//     serialize_into(
//         &mut f,
//         &CachedWeights {
//             particle: weight_window.particle,
//             weight_error,
//         },
//     )
//     .expect("Failed to serialize cached weights to binary");
// }
