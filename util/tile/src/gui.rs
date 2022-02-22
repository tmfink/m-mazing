use std::sync::mpsc::TryRecvError;

use bevy::app::AppExit;

use crate::*;

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut should_refresh: ResMut<RefreshTile>,
    mut ctx: ResMut<Ctx>,
    tile: Option<ResMut<CurrentTile>>,
    mut availability: ResMut<TileAvailability>,
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

    if keyboard_input.any_just_pressed([KeyCode::K, KeyCode::U]) {
        if let Some(tile) = tile {
            availability.0 = match availability.0 {
                CellItemAvailability::Available => CellItemAvailability::Used,
                CellItemAvailability::Used => CellItemAvailability::Available,
            };
            info!("availability = {:?}", availability.0);
        }
    }
}

/*
pub fn update(ctx: &mut Ctx) {

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


    // todo: smarter fit rect for all tiles
    let fit_rect = Rect {
        x: -3.,
        y: -3.,
        w: 6.,
        h: 6.,
    };
    let whole_camera = camera_zoom_to_fit(fit_rect);
    set_camera(&whole_camera);


    if keyboard_input.just_pressed(mq::KeyCode::P) {
        println!("{:#?}", tile);
    }
}
*/

/*
pub fn draw(ctx: NonSend<Ctx>, render: Res<RenderState>, mut commands: Commands) {
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
}
*/

pub fn spawn_tile(
    ctx: Res<Ctx>,
    availability: Res<TileAvailability>,
    render: Res<RenderState>,
    refresh: Res<RefreshTile>,
    tile: Option<ResMut<CurrentTile>>,
    mut commands: Commands,
    mut query: Query<&mut Text, With<TitleString>>,
) {
    if !(refresh.0 || ctx.is_changed() || availability.is_changed()) {
        return;
    }

    info!("spawning tile");

    if let Some(tile) = tile {
        commands.entity(tile.id).despawn_recursive();
    }

    let title_str = &mut query.single_mut().sections[0].value;
    let tile = if let Some(item) = ctx.tileset.get(ctx.tile_idx as usize) {
        *title_str = format!(
            "TILE: {} (idx={}, avail={:?})",
            item.0, ctx.tile_idx, availability.0
        );
        &item.1
    } else {
        *title_str = "no tile".to_string();
        return;
    };

    /*
    if let Some((tile_name, tile)) = tile {
        ...
        if keyboard_input.just_pressed(mq::KeyCode::LeftBracket) {
            tile.rotate(SpinDirection::CounterClockwise);
        }
        if keyboard_input.just_pressed(mq::KeyCode::RightBracket) {
            tile.rotate(SpinDirection::Clockwise);
        }
        ...
    }
    */

    let mut tile = tile.clone();
    for cell in tile.cells_iter_mut() {
        cell.set_availability(availability.0);
    }

    let id = tile.spawn(Vec2::ZERO, &*render, &mut commands);

    let new_tile = CurrentTile { id, tile };
    commands.insert_resource(new_tile);
}
