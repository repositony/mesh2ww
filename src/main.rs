//! Command line generation of weight windows
#![doc(hidden)]

mod cli;
mod conversion;
mod logging;
mod parser;
// mod update;
mod wrappers;

// internal modules
use crate::cli::help_wanted;

// neutronics toolbox
use ntools::weights::write_multi_particle;

// other crates
use anyhow::{anyhow, Result};
use log::{debug, info};

// Convenience types
type ArgSet = Vec<String>;

fn main() -> Result<()> {
    // short circuit for help messages
    if help_wanted() {
        return Ok(());
    }

    // set up logging (Info is the default)
    logging::init_logging()?;

    // split up the command line args by the '+' delimeter and parse each one
    // through Clap to verify the arguments
    debug!("Parsing command line sets");
    let ww_config_sets = parser::ww_config();
    if ww_config_sets.is_empty() {
        return Err(anyhow!("No valid meshtal files were found"));
    }

    // collect up all weight windows, just exclude any missing and warn the user
    debug!("Generating weight windows");
    let particle_weights = conversion::collect_weight_windows(ww_config_sets)?;

    // Write the weight window file
    let file_config = parser::file_config();
    info!("Writing WWINP file");
    debug!("Ouput file: \"{}\"", file_config.output);
    write_multi_particle(&particle_weights, &file_config.output, !file_config.trim);

    info!("Conversion complete");
    Ok(())
}
