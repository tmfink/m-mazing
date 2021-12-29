use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use log::Level;
use macroquad::logging::*;

#[cfg(feature = "gui")]
use macroquad::prelude as mq;

use m_core::{render::*, tile::Tile};
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
    let render = m_core::render::RenderState::default();

    debug!("tileset: {:#?}", ctx.tileset);
    let tile = ctx.tileset.first();

    loop {
        mq::clear_background(render.theme.bg_color);

        if mq::is_key_pressed(mq::KeyCode::Q) | mq::is_key_pressed(mq::KeyCode::Escape) {
            break;
        }

        // todo: smarter camera set (zoom to fill while preserving aspect ratio)
        mq::set_camera(&mq::Camera2D::from_display_rect(mq::Rect {
            x: -3.,
            y: -3.,
            w: 6.,
            h: 6.,
        }));

        let text = if let Some((tile_name, tile)) = tile {
            tile.render(mq::Vec2::default(), &render);
            format!("TILE: {}", tile_name)
        } else {
            "no tile".to_string()
        };

        // screen space camera for text
        mq::set_default_camera();
        let (font_size, font_scale, font_scale_aspect) =
            mq::camera_font_scale(render.theme.font_size);
        draw_text_align(
            &text,
            AlignHoriz::Center,
            AlignVert::Bottom,
            mq::TextParams {
                color: render.theme.font_color,
                font_size,
                font_scale,
                font_scale_aspect,
                font: Default::default(),
            },
        );
        mq::next_frame().await
    }

    Ok(())
}
