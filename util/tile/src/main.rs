use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use cfg_if::cfg_if;
use clap::Parser;

use m_mazing_core::prelude::*;

cfg_if! {
    if #[cfg(feature = "gui")] {
        use m_mazing_core::macroquad::prelude as mq;
        use m_mazing_core::macroquad;
    }
}

#[cfg(feature = "gui")]
const LEGEND: &str = "
Arrow keys - cycle between tiles;
K/U - toggle cell availability
";

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
struct Ctx {
    args: Args,
    tileset: Vec<(String, Tile)>,
}

fn get_ctx() -> Result<Ctx> {
    let mut args = Args::parse();
    args.verbose.set_default(Some(log::Level::Info));

    let mut tile_input_file = File::open(&args.tile_file)
        .with_context(|| format!("Failed to open input file {:?}", &args.tile_file))?;
    let mut tile_str = String::new();
    tile_input_file
        .read_to_string(&mut tile_str)
        .with_context(|| "Failed to read input")?;
    let tileset = m_mazing_core::tile::tileset::tileset_from_str(&tile_str)
        .with_context(|| "failed to parse tileset")?;

    Ok(Ctx { args, tileset })
}

#[cfg(not(feature = "gui"))]
fn main() -> Result<()> {
    let ctx = get_ctx().with_context(|| "Failed to generate context")?;

    #[cfg(any(not(feature = "gui"), feature = "logs-rs"))]
    init_logging(&ctx.args);

    info!("tileset = {:#?}", ctx.tileset);

    Ok(())
}

// todo: manual mq::Window::new() to support anyhow::Result
#[cfg(feature = "gui")]
#[macroquad::main("M-Mazing Tile Util")]
async fn main() -> Result<()> {
    let mut ctx = get_ctx().with_context(|| "Failed to generate context")?;
    let render = RenderState::default();

    #[cfg(feature = "log-rs")]
    init_logging(&ctx.args);

    info!("tileset: {:#?}", ctx.tileset);
    let mut tile_idx: isize = 0;
    let mut availability = CellItemAvailability::Available;

    loop {
        mq::clear_background(render.theme.bg_color);

        if mq::is_key_pressed(mq::KeyCode::Q) || mq::is_key_pressed(mq::KeyCode::Escape) {
            break;
        }

        if mq::is_key_pressed(mq::KeyCode::Right) || mq::is_key_pressed(mq::KeyCode::Down) {
            tile_idx += 1;
        }
        if mq::is_key_pressed(mq::KeyCode::Left) || mq::is_key_pressed(mq::KeyCode::Up) {
            tile_idx -= 1;
        }
        tile_idx = tile_idx.rem_euclid(ctx.tileset.len() as isize);
        let tile = ctx.tileset.get_mut(tile_idx as usize);

        if mq::is_key_pressed(mq::KeyCode::K) || mq::is_key_pressed(mq::KeyCode::U) {
            availability = match availability {
                CellItemAvailability::Available => CellItemAvailability::Used,
                CellItemAvailability::Used => CellItemAvailability::Available,
            };
        }

        // todo: smarter fit rect for all tiles
        let fit_rect = mq::Rect {
            x: -3.,
            y: -3.,
            w: 6.,
            h: 6.,
        };
        let whole_camera = camera_zoom_to_fit(fit_rect);
        mq::set_camera(&whole_camera);

        let text = if let Some((tile_name, tile)) = tile {
            for cell in tile.cells_mut() {
                cell.set_availability(availability);
            }
            tile.render(mq::Vec2::default(), &render);
            format!(
                "TILE: {} (idx={}, avail={:?})",
                tile_name, tile_idx, availability
            )
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

        draw_text_align(
            LEGEND.trim(),
            AlignHoriz::Left,
            AlignVert::Top,
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
