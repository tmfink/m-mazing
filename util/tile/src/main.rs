use std::{fs::File, io::Read, path::PathBuf};

use log::*;
//use m_mazing_core::*;

use anyhow::{Context, Result};
use clap::{ArgEnum, Parser};

/// Utility to debug Tiles
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Log verbosity
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Format to render
    #[clap(arg_enum, long, short, default_value = "Render::TextDebug")]
    render: Render,

    /// File (defaults to stdin) with tiles
    #[clap(long, short)]
    tile_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ArgEnum)]
#[clap(rename_all = kebab-case)]
enum Render {
    TextDebug,
}

fn init_logging(args: &Args) {
    let level = match args.verbose.log_level() {
        None => LevelFilter::Off,
        Some(level) => match level {
            Level::Error => LevelFilter::Error,
            Level::Warn => LevelFilter::Warn,
            Level::Info => LevelFilter::Info,
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
        },
    };
    simple_logger::SimpleLogger::new()
        .with_level(level)
        .init()
        .expect("Failed to init logging");
    info!("log verbosity: {:?}", level);
}

fn input_file(file: &Option<PathBuf>) -> Result<Box<dyn Read>> {
    Ok(match file {
        None => Box::new(std::io::stdin()),
        Some(path) => Box::new(File::open(path)?),
    })
}

fn main() -> Result<()> {
    let args = Args::parse();
    init_logging(&args);

    let mut tile_input_file = input_file(&args.tile_file)
        .with_context(|| format!("Failed to open input file {:?}", &args.tile_file))?;
    let mut tile_str = String::new();
    tile_input_file
        .read_to_string(&mut tile_str)
        .with_context(|| "Failed to read input")?;
    let tileset = m_mazing_core::tile::tileset::tileset_from_str(&tile_str)
        .with_context(|| "failed to parse tileset")?;
    println!("tileset: {:#?}", tileset);

    Ok(())
}
