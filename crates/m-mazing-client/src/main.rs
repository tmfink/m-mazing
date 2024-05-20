use anyhow::Result;
use bevy::prelude::*;
use clap::Parser;

use m_mazing_core::{
    bevy::{self, log::LogPlugin},
    log_level,
    prelude::*,
};

/// Utility to debug Tiles
#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
pub struct Args {
    /// Log verbosity
    #[clap(long, short, parse(from_occurrences))]
    verbose: i32,

    /// Quiet log
    #[clap(long, short, parse(from_occurrences), conflicts_with = "verbose")]
    quiet: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let level = log_level(args.verbose, args.quiet);

    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level,
            ..Default::default()
        }))
        .add_plugins(ShapePlugin)
        .run();

    Ok(())
}
