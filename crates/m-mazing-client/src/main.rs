use anyhow::Result;
use bevy::prelude::*;
use clap::Parser;

use m_mazing_core::{
    bevy::{self, log::LogPlugin},
    bevy_prototype_lyon::plugin::ShapePlugin,
    log_level,
};

/// Game client
#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
pub struct Args {
    /// Log verbosity
    #[clap(long, short, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Quiet log
    #[clap(long, short, action = clap::ArgAction::Count, conflicts_with = "verbose")]
    quiet: u8,
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
