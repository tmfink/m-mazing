use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use cfg_if::cfg_if;
use clap::Parser;

use m_mazing_core::prelude::*;

cfg_if! {
    if #[cfg(feature = "gui")] {
        use m_mazing_core::macroquad::prelude as mq;
        use m_mazing_core::macroquad;

        mod gui;
        use crate::gui::*;
    }
}

/// Utility to debug Tiles
#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
pub struct Args {
    /// Log verbosity
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// File with tile data
    #[clap(long, short)]
    tile_file: PathBuf,
}

#[cfg(any(not(feature = "gui"), feature = "log-rs"))]
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
        .with_utc_timestamps()
        .init()
        .expect("Failed to init logging");
    info!("log verbosity: {:?}", level);
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Ctx {
    pub args: Args,
    pub tileset: Vec<(String, Tile)>,
    pub tile_idx: isize,
    pub availability: CellItemAvailability,
    pub text: String,
}

impl Ctx {
    fn new() -> Result<Ctx> {
        let mut args = Args::parse();
        args.verbose.set_default(Some(log::Level::Info));

        let mut ctx = Ctx {
            args,
            tileset: Default::default(),
            tile_idx: 0,
            availability: CellItemAvailability::Available,
            text: String::new(),
        };
        ctx.refresh()?;
        Ok(ctx)
    }

    fn refresh(&mut self) -> Result<()> {
        let mut tile_input_file = File::open(&self.args.tile_file)
            .with_context(|| format!("Failed to open input file {:?}", &self.args.tile_file))?;
        let mut tile_str = String::new();
        tile_input_file
            .read_to_string(&mut tile_str)
            .with_context(|| "Failed to read input")?;
        self.tileset = m_mazing_core::tile::tileset::tileset_from_str(&tile_str)
            .with_context(|| "failed to parse tileset")?;

        Ok(())
    }
}

#[cfg(not(feature = "gui"))]
fn main() -> Result<()> {
    let ctx = Ctx::new().with_context(|| "Failed to generate context")?;

    #[cfg(any(not(feature = "gui"), feature = "logs-rs"))]
    init_logging(&ctx.args);

    info!("tileset = {:#?}", ctx.tileset);

    Ok(())
}

// todo: manual mq::Window::new() to support anyhow::Result
#[cfg(feature = "gui")]
#[macroquad::main("M-Mazing Tile Util")]
async fn main() -> Result<()> {
    let mut ctx = Ctx::new().with_context(|| "Failed to generate context")?;
    let render = RenderState::default();

    #[cfg(feature = "log-rs")]
    init_logging(&ctx.args);

    info!("tileset: {:#?}", ctx.tileset);

    loop {
        mq::clear_background(render.theme.bg_color);

        match update(&mut ctx) {
            Continuation::Continue => (),
            Continuation::Exit => break,
        }
        draw(&ctx, &render);

        mq::next_frame().await
    }

    Ok(())
}
