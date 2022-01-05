use std::{fs::File, io::Read, path::PathBuf, sync::mpsc};

use anyhow::{Context, Result};
use cfg_if::cfg_if;
use clap::Parser;

use m_mazing_core::prelude::*;
use notify::Watcher;

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

    /// Start idx
    #[clap(long = "start-idx", short = 'i', default_value = "0")]
    index: usize,
}

#[cfg(feature = "log-rs")]
fn init_logging(args: &Args) {
    use log::*;

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
#[derive(Debug)]
pub struct Ctx {
    pub args: Args,
    pub tileset: Vec<(String, Tile)>,
    pub tile_idx: isize,
    pub availability: CellItemAvailability,
    pub text: String,
    pub notify_rx: mpsc::Receiver<notify::Result<notify::Event>>,
    pub notify_watcher: notify::RecommendedWatcher,
}

impl Ctx {
    fn new() -> Result<Ctx> {
        let mut args = Args::parse();
        args.verbose.set_default(Some(log::Level::Info));

        let (notify_tx, notify_rx) = mpsc::channel();
        let mut notify_watcher = notify::RecommendedWatcher::new(notify_tx)
            .context("Failed to create notify watcher")?;
        notify_watcher
            .watch(&args.tile_file, notify::RecursiveMode::Recursive)
            .context(format!("Failed to watch file {:?}", args.tile_file))?;

        let tile_idx = args.index as isize;
        let mut ctx = Ctx {
            args,
            tileset: Default::default(),
            tile_idx,
            availability: CellItemAvailability::Available,
            text: String::new(),
            notify_rx,
            notify_watcher,
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

    #[cfg(feature = "log-rs")]
    init_logging(&ctx.args);

    info!("tileset = {:#?}", ctx.tileset);

    Ok(())
}

// todo: manual mq::Window::new() to support anyhow::Result
#[cfg(feature = "gui")]
#[macroquad::main("M-Mazing Tile Util")]
async fn main() -> Result<()> {
    let mut ctx = Ctx::new()
        .with_context(|| "Failed to generate context")
        // bug: macroquad does not "forward" result to real main(), so unwrap() for now
        .unwrap();
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
