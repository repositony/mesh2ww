//! Handles parsing the command line arguments into something useful

use crate::cli::{cli_init, is_flag_present};
use crate::wrappers::{CliByteOrder, CliCompressor, CliVtkFormat};
use crate::ArgSet;

use anyhow::{anyhow, Result};
use clap::ArgMatches;
use log::warn;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct WWConfig {
    pub meshtal: String,
    pub number: u32,
    pub power: Vec<f64>,
    pub error: Vec<f64>,
    pub total: bool,
    pub scale: f64,
}

#[derive(Debug)]
pub struct VtkConfig {
    pub vtk: bool,
    pub format: CliVtkFormat,
    pub compressor: CliCompressor,
    pub endian: CliByteOrder,
    pub resolution: u8,
}

#[derive(Debug)]
pub struct FileConfig {
    pub trim: bool,
    pub output: String,
}

pub fn ww_config() -> Vec<WWConfig> {
    split_argument_sets()
        .iter()
        .filter_map(|arg_set| {
            let cli = ww_set(arg_set.to_owned());
            if let Err(e) = &cli {
                warn!("{:?}, skipping", e);
            }
            cli.ok()
        })
        .collect::<Vec<WWConfig>>()
}

pub fn vtk_config() -> VtkConfig {
    let matches = all_argument_matches();

    // fine to unwrap these matches because a default has been set
    VtkConfig {
        vtk: is_flag_present(&["--vtk"]),
        format: matches
            .iter()
            .find_map(|m| m.get_one::<CliVtkFormat>("format").cloned())
            .unwrap_or(CliVtkFormat::Xml),
        compressor: matches
            .iter()
            .find_map(|m| m.get_one::<CliCompressor>("compressor").cloned())
            .unwrap_or(CliCompressor::LZMA),
        endian: matches
            .iter()
            .find_map(|m| m.get_one::<CliByteOrder>("endian").cloned())
            .unwrap_or(CliByteOrder::BigEndian),
        resolution: matches
            .iter()
            .find_map(|m| m.get_one::<u8>("resolution").cloned())
            .unwrap_or(1),
    }
}

pub fn file_config() -> FileConfig {
    let matches = all_argument_matches();

    // fine to unwrap these matches because a default has been set
    FileConfig {
        trim: is_flag_present(&["--trim"]),
        output: matches
            .iter()
            .find_map(|m| m.get_one::<String>("output").cloned())
            .unwrap_or("wwinp".to_string()),
    }
}

fn split_argument_sets() -> Vec<ArgSet> {
    let name = env::args().next().unwrap();
    let raw_args = env::args().skip(1).collect::<Vec<String>>();
    let mut tallies = Vec::with_capacity(37);

    for s in raw_args.split(|p| p == "+") {
        tallies.push(std::iter::once(name.clone()).chain(s.to_owned()).collect());
    }

    tallies
}

fn powers_vector(matches: &mut ArgMatches) -> Vec<f64> {
    matches
        .remove_many::<f64>("power")
        .unwrap_or_default()
        .collect()
}

fn errors_vector(matches: &mut ArgMatches) -> Vec<f64> {
    matches
        .remove_many::<f64>("error")
        .unwrap_or_default()
        .collect()
}

fn all_argument_matches() -> Vec<ArgMatches> {
    split_argument_sets()
        .iter()
        .map(|set| cli_init().get_matches_from(set))
        .collect()
}

fn ww_set(arguments: Vec<String>) -> Result<WWConfig> {
    let mut matches = cli_init().get_matches_from(arguments);

    let meshtal: Option<String> = matches.try_remove_one("meshtal")?;
    let number: Option<u32> = matches.try_remove_one("number")?;

    match meshtal {
        Some(_) => {
            // quickly check if all the files even exist
            if !Path::new(&meshtal.clone().unwrap()).exists() {
                return Err(anyhow!("Unable to find file \"{}\"", &meshtal.unwrap()));
            }
        }
        None => return Err(anyhow!("Empty <meshtal> positional argument in set")),
    }

    // fine to unwrap these matches because a default has been set
    Ok(WWConfig {
        meshtal: meshtal.unwrap(),
        number: number.unwrap(),
        power: powers_vector(&mut matches),
        error: errors_vector(&mut matches),
        total: matches.remove_one("total").unwrap(),
        scale: matches.remove_one("scale").unwrap(),
    })
}
