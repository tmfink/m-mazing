use std::{fs::File, io::Read, path::PathBuf, sync::mpsc};

use anyhow::{Context, Result};
use clap::Parser;

use m_mazing_core::bevy;
use m_mazing_core::bevy::log::LogPlugin;
use m_mazing_core::prelude::*;
use m_mazing_core::render::RenderState;
use notify::Watcher;

use bevy::ecs as bevy_ecs; // needed for Component derive
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

mod gui;
use crate::gui::*;

const LEGEND: &str = "
Arrow keys - cycle;
K/U - toggle cell used;
[] - rotate;
P - print;
R - reload;
Home/End - start/end;
";

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

    /// File with tile data
    #[clap(long, short)]
    tile_file: PathBuf,

    /// Start idx
    #[clap(long = "start-idx", short = 'i', default_value = "0")]
    index: usize,
}
#[derive(Debug, Resource)]
pub struct CurrentTile {
    pub tile: Tile,
    pub id: Entity,
}

#[derive(Debug, Default, Resource)]
pub struct RefreshTile(pub bool);

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ctx {
    pub args: Args,
    pub tileset: Vec<(String, Tile)>,
    pub tile_idx: isize,
    pub notify_rx: mpsc::Receiver<notify::Result<notify::Event>>,
    pub notify_watcher: notify::RecommendedWatcher,
}

#[derive(Component)]
pub struct TitleString;

#[derive(Debug, Resource)]
pub struct TileAvailability(pub CellItemAvailability);

#[derive(Debug, Default, Resource)]
pub struct TileRotation {
    pub left_turns: u8,
}

impl Default for TileAvailability {
    fn default() -> Self {
        Self(CellItemAvailability::Available)
    }
}

impl Ctx {
    fn new() -> Result<Ctx> {
        let args = Args::parse();

        let (notify_tx, notify_rx) = mpsc::channel();
        let mut notify_watcher =
            notify::RecommendedWatcher::new(notify_tx, notify::Config::default())
                .context("Failed to create notify watcher")?;
        notify_watcher
            .watch(&args.tile_file, notify::RecursiveMode::Recursive)
            .context(format!("Failed to watch file {:?}", args.tile_file))?;

        let tile_idx = args.index as isize;
        let mut ctx = Ctx {
            args,
            tileset: Default::default(),
            tile_idx,
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

fn setup_system(mut commands: Commands) {
    const CAMERA_EXTENT: f32 = 3.0;
    let mut camera_bundle = Camera2dBundle::default();

    // able to re-size window if pop out **after** moving window
    camera_bundle.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: CAMERA_EXTENT,
        min_height: CAMERA_EXTENT,
    };

    // hack to modify camera
    camera_bundle.transform.scale = Vec3::new(3.0, 3.0, 1.0);

    commands.spawn(camera_bundle);
}

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    background_color: Color::NONE.into(),

                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                ..Default::default()
                            },
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 50.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(TitleString);
                });
        });

    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::from_section(
            LEGEND.trim().to_string(),
            TextStyle {
                font,
                font_size: 40.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum MySystemSet {
    Input,
    SpawnTile,
}

fn frame_init(mut refresh: ResMut<RefreshTile>) {
    refresh.0 = false;
}

#[allow(unused)]
fn debug_system(query: Query<Entity>) {
    info!("entities: {}", query.iter().count());
}

fn main() -> Result<()> {
    let ctx = Ctx::new().with_context(|| "Failed to generate context")?;
    let level = log_level(ctx.args.verbose, ctx.args.quiet);

    println!("tileset: {:#?}", ctx.tileset);

    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_non_send_resource(ctx)
        .init_resource::<RenderState>()
        .init_resource::<TileAvailability>()
        .insert_resource(RefreshTile(true))
        .init_resource::<TileRotation>()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    asset_folder: "../../assets".to_string(),
                    ..default()
                })
                .set(LogPlugin {
                    level,
                    ..Default::default()
                }),
        )
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_startup_system(ui_setup)
        .add_system(frame_init.before(MySystemSet::Input))
        .add_system(keyboard_input_system.in_set(MySystemSet::Input))
        .add_system(notify_tileset_change.in_set(MySystemSet::Input))
        .add_system(
            spawn_tile
                .in_set(MySystemSet::SpawnTile)
                .after(MySystemSet::Input),
        )
        .add_system(print_tile.after(MySystemSet::SpawnTile))
        //.add_system(debug_system)
        .run();

    Ok(())
}
