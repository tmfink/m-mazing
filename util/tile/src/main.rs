use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use log::Level;
use macroquad::logging::*;

#[cfg(feature = "gui")]
use macroquad::prelude as mq;

use m_core::tile::Tile;
use m_mazing_core as m_core;

/// Utility to debug Tiles
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Log verbosity
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// File with tile data
    #[clap(long, short)]
    tile_file: PathBuf,
}

#[cfg(not(feature = "gui"))]
fn init_logging(args: &Args) {
    use log::LevelFilter;

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

struct Ctx {
    _args: Args,
    tileset: Vec<(String, Tile)>,
}

fn get_ctx() -> Result<Ctx> {
    let mut _args = Args::parse();
    _args.verbose.set_default(Some(Level::Info));

    let mut tile_input_file = File::open(&_args.tile_file)
        .with_context(|| format!("Failed to open input file {:?}", &_args.tile_file))?;
    let mut tile_str = String::new();
    tile_input_file
        .read_to_string(&mut tile_str)
        .with_context(|| "Failed to read input")?;
    let tileset = m_mazing_core::tile::tileset::tileset_from_str(&tile_str)
        .with_context(|| "failed to parse tileset")?;

    Ok(Ctx { _args, tileset })
}

#[cfg(not(feature = "gui"))]
fn main() -> Result<()> {
    let ctx = get_ctx().with_context(|| "Failed to generate context")?;
    init_logging(&ctx._args);
    print!("tileset: {:#?}", ctx.tileset);

    Ok(())
}

#[cfg(feature = "gui")]
#[macroquad::main("M-Mazing Tile Util")]
async fn main() -> Result<()> {
    let ctx = get_ctx().with_context(|| "Failed to generate context")?;
    debug!("tileset: {:#?}", ctx.tileset);
    loop {
        mq::clear_background(mq::Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        });
        mq::next_frame().await
    }
}
