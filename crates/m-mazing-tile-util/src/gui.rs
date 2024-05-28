use std::sync::mpsc::TryRecvError;

use bevy::app::AppExit;
use m_mazing_core::render::RenderState;

use crate::*;

pub fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut should_refresh: ResMut<RefreshTile>,
    mut ctx: NonSendMut<Ctx>,
    mut availability: ResMut<TileAvailability>,
    mut tile_rotation: ResMut<TileRotation>,
) {
    if keyboard_input.any_pressed([KeyCode::Escape, KeyCode::KeyQ]) {
        app_exit_events.send(AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        should_refresh.0 = true;
        match ctx.refresh() {
            Ok(()) => info!("Manually reloading"),
            Err(err) => error!("Failed to manually reload: {:#}", err),
        }
    }

    if keyboard_input.any_just_pressed([KeyCode::ArrowRight, KeyCode::ArrowDown]) {
        ctx.tile_idx += 1;
    }
    if keyboard_input.any_just_pressed([KeyCode::ArrowLeft, KeyCode::ArrowUp]) {
        ctx.tile_idx -= 1;
    }

    // Avoid triggering change detection if unchanged
    let new_tile_idx = ctx
        .tile_idx
        .checked_rem_euclid(ctx.tileset.len() as isize)
        .unwrap_or(0);
    if new_tile_idx != ctx.tile_idx {
        ctx.tile_idx = new_tile_idx;
    }

    if keyboard_input.just_pressed(KeyCode::Home) {
        ctx.tile_idx = 0;
    }
    if keyboard_input.just_pressed(KeyCode::End) {
        ctx.tile_idx = ctx.tileset.len() as isize - 1;
    }

    if keyboard_input.any_just_pressed([KeyCode::KeyK, KeyCode::KeyU]) {
        availability.0 = match availability.0 {
            CellItemAvailability::Available => CellItemAvailability::Used,
            CellItemAvailability::Used => CellItemAvailability::Available,
        };
        info!("availability = {:?}", availability.0);
    }

    const NUM_SPIN_DIRS: u8 = 4;
    if keyboard_input.any_just_pressed([KeyCode::BracketLeft]) {
        tile_rotation.left_turns = (tile_rotation.left_turns + 1).rem_euclid(NUM_SPIN_DIRS);
    }
    if keyboard_input.any_just_pressed([KeyCode::BracketRight]) {
        tile_rotation.left_turns =
            (tile_rotation.left_turns as i8 - 1).rem_euclid(NUM_SPIN_DIRS as i8) as u8;
    }
}

pub fn notify_tileset_change(mut should_refresh: ResMut<RefreshTile>, mut ctx: NonSendMut<Ctx>) {
    match ctx.notify_rx.try_recv() {
        Ok(Ok(event)) => {
            info!("new event {:?}", event);
            should_refresh.0 = true;

            match ctx.refresh() {
                Ok(()) => info!("Manually reloading"),
                Err(err) => error!("Failed to manually reload: {:#}", err),
            }
        }
        Ok(Err(err)) => error!("Failed to get new event {:#}", err),
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => error!("Notify disconnected"),
    }
}

/*
pub fn update(ctx: &mut Ctx) {
    // todo: smarter fit rect for all tiles
    let fit_rect = Rect {
        x: -3.,
        y: -3.,
        w: 6.,
        h: 6.,
    };
    let whole_camera = camera_zoom_to_fit(fit_rect);
    set_camera(&whole_camera);
}
*/

/*
pub fn draw(ctx: NonSend<Ctx>, render: Res<RenderState>, mut commands: Commands) {
    // screen space camera for text
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(render.theme.font_size);
}
*/

#[allow(clippy::too_many_arguments)]
pub fn spawn_tile(
    ctx: NonSend<Ctx>,
    availability: Res<TileAvailability>,
    render: Res<RenderState>,
    refresh: Res<RefreshTile>,
    tile: Option<ResMut<CurrentTile>>,
    tile_rotation: Res<TileRotation>,
    mut commands: Commands,
    mut query: Query<&mut Text, With<TitleString>>,
) {
    if !(refresh.0 || ctx.is_changed() || availability.is_changed() || tile_rotation.is_changed()) {
        return;
    }

    info!("spawning tile");

    if let Some(tile) = tile {
        commands.entity(tile.id).despawn_recursive();
    }

    let title_str = &mut query.single_mut().sections[0].value;
    let tile = if let Some(item) = ctx.tileset.get(ctx.tile_idx as usize) {
        *title_str = format!(
            "TILE: {} (idx={})\navail={:?}, left_turns={}",
            item.0, ctx.tile_idx, availability.0, tile_rotation.left_turns
        );
        &item.1
    } else {
        *title_str = "no tile".to_string();
        return;
    };

    let mut tile = tile.clone();
    for cell in tile.cells_iter_mut() {
        cell.set_availability(availability.0);
    }
    for _ in 0..tile_rotation.left_turns {
        tile.rotate(SpinDirection::CounterClockwise);
    }

    let id = tile.spawn(Vec2::ZERO, &render, &mut commands);

    let new_tile = CurrentTile { id, tile };
    commands.insert_resource(new_tile);
}

pub fn print_tile(keyboard_input: Res<ButtonInput<KeyCode>>, tile: Option<Res<CurrentTile>>) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        match tile {
            None => println!("No tile"),
            Some(tile) => println!("{:#?}", tile.tile),
        }
    }
}
