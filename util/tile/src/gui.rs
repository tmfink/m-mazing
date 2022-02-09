use std::sync::mpsc::TryRecvError;

use bevy::app::AppExit;
use bevy::input::{keyboard::KeyboardInput, ElementState};

use crate::*;

const LEGEND: &str = "
Arrow keys - cycle;
K/U - toggle cell used;
[] - rotate;
P - print;
R - reload;
Home/End - start/end;
";

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut should_refresh: ResMut<RefreshTile>,
    mut ctx: ResMut<Ctx>,
) {
    if keyboard_input.any_pressed([KeyCode::Escape, KeyCode::Q]) {
        app_exit_events.send(AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::R) {
        should_refresh.0 = true;
    }

    if keyboard_input.any_just_pressed([KeyCode::Right, KeyCode::Down]) {
        ctx.tile_idx += 1;
    }
    if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Up]) {
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
}

pub fn update(ctx: &mut Ctx) {

    /*
    match ctx.notify_rx.try_recv() {
        Ok(Ok(event)) => {
            info!("new event {:?}", event);
            should_refresh = true
        }
        Ok(Err(err)) => error!("Failed to get new event {:#}", err),
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => error!("Notify disconnected"),
    }
    if should_refresh {
        match ctx.refresh() {
            Ok(()) => info!("Refreshed ctx"),
            Err(err) => error!("Failed to refresh: {:#}", err),
        }
    }


    if keyboard_input.just_pressed(mq::KeyCode::K) || mq::keyboard_input.just_pressed(mq::KeyCode::U) {
        ctx.availability = match ctx.availability {
            CellItemAvailability::Available => CellItemAvailability::Used,
            CellItemAvailability::Used => CellItemAvailability::Available,
        };
    }

    // todo: smarter fit rect for all tiles
    let fit_rect = Rect {
        x: -3.,
        y: -3.,
        w: 6.,
        h: 6.,
    };
    let whole_camera = camera_zoom_to_fit(fit_rect);
    set_camera(&whole_camera);

    if let Some((tile_name, tile)) = tile {
        for cell in tile.cells_iter_mut() {
            cell.set_availability(ctx.availability);
        }
        if keyboard_input.just_pressed(mq::KeyCode::LeftBracket) {
            tile.rotate(SpinDirection::CounterClockwise);
        }
        if keyboard_input.just_pressed(mq::KeyCode::RightBracket) {
            tile.rotate(SpinDirection::Clockwise);
        }
        ctx.text = format!(
            "TILE: {} (idx={}, avail={:?})",
            tile_name, ctx.tile_idx, ctx.availability
        );
    } else {
        ctx.text = "no tile".to_string();
    }

    if keyboard_input.just_pressed(mq::KeyCode::P) {
        println!("{:#?}", tile);
    }

    Continuation::Continue
    */
}

pub fn draw(ctx: NonSend<Ctx>, render: Res<RenderState>, mut commands: Commands) {
    //if let Some((_tile_name, tile)) = ctx.tileset.get(ctx.tile_idx as usize) {
    //    tile.render(Vec2::default(), &render, &mut commands);
    //}

    /*
    // screen space camera for text
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(render.theme.font_size);
    draw_text_align(
        &ctx.text,
        AlignHoriz::Center,
        AlignVert::Bottom,
        TextParams {
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
        TextParams {
            color: render.theme.font_color,
            font_size,
            font_scale,
            font_scale_aspect,
            font: Default::default(),
        },
    );
    */
}

pub fn spawn_tile(
    ctx: Res<Ctx>,
    render: Res<RenderState>,
    refresh: Res<RefreshTile>,
    mut tile: Option<ResMut<CurrentTile>>,
    mut commands: Commands,
) {
    if !(refresh.0 || ctx.is_changed()/* || tile.is_none() */) {
        return;
    }

    info!("spawning tile");

    if let Some(tile) = tile {
        commands.entity(tile.id).despawn_recursive();
    }

    let (name, tile) = if let Some(item) = ctx.tileset.get(ctx.tile_idx as usize) {
        item
    } else {
        return;
    };

    let tile = tile.clone();
    let id = tile.spawn(Vec2::ZERO, &*render, &mut commands);

    let new_tile = CurrentTile { id, tile };
    commands.insert_resource(new_tile);
}
