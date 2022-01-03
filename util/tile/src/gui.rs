use std::sync::mpsc::TryRecvError;

use crate::*;

const LEGEND: &str = "
Arrow keys - cycle;
K/U - toggle cell used;
[] - rotate
P - print tile
R - reload tileset
";

pub enum Continuation {
    Continue,
    Exit,
}

#[must_use]
pub fn update(ctx: &mut Ctx) -> Continuation {
    if mq::is_key_pressed(mq::KeyCode::Q) || mq::is_key_pressed(mq::KeyCode::Escape) {
        return Continuation::Exit;
    }

    let mut should_refresh = false;
    if mq::is_key_pressed(mq::KeyCode::R) {
        should_refresh = true
    }
    match ctx.notify_rx.try_recv() {
        Ok(Ok(event)) => {
            info!("new event {:?}", event);
            should_refresh = true
        }
        Ok(Err(err)) => error!("Failed to get new event {}", err),
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => error!("Notify disconnected"),
    }
    if should_refresh {
        match ctx.refresh() {
            Ok(()) => info!("Refreshed ctx"),
            Err(err) => error!("Failed to refresh: {}", err),
        }
    }

    if mq::is_key_pressed(mq::KeyCode::Right) || mq::is_key_pressed(mq::KeyCode::Down) {
        ctx.tile_idx += 1;
    }
    if mq::is_key_pressed(mq::KeyCode::Left) || mq::is_key_pressed(mq::KeyCode::Up) {
        ctx.tile_idx -= 1;
    }
    ctx.tile_idx = ctx
        .tile_idx
        .checked_rem_euclid(ctx.tileset.len() as isize)
        .unwrap_or(0);
    let tile = ctx.tileset.get_mut(ctx.tile_idx as usize);

    if mq::is_key_pressed(mq::KeyCode::K) || mq::is_key_pressed(mq::KeyCode::U) {
        ctx.availability = match ctx.availability {
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

    if let Some((tile_name, tile)) = tile {
        for cell in tile.cells_mut() {
            cell.set_availability(ctx.availability);
        }
        if mq::is_key_pressed(mq::KeyCode::LeftBracket) {
            tile.rotate(SpinDirection::CounterClockwise);
        }
        if mq::is_key_pressed(mq::KeyCode::RightBracket) {
            tile.rotate(SpinDirection::Clockwise);
        }
        ctx.text = format!(
            "TILE: {} (idx={}, avail={:?})",
            tile_name, ctx.tile_idx, ctx.availability
        );
    } else {
        ctx.text = "no tile".to_string();
    }

    if mq::is_key_pressed(mq::KeyCode::P) {
        println!("{:#?}", tile);
    }

    Continuation::Continue
}

pub fn draw(ctx: &Ctx, render: &RenderState) {
    if let Some((_tile_name, tile)) = ctx.tileset.get(ctx.tile_idx as usize) {
        tile.render(mq::Vec2::default(), render);
    }
    // screen space camera for text
    mq::set_default_camera();
    let (font_size, font_scale, font_scale_aspect) = mq::camera_font_scale(render.theme.font_size);
    draw_text_align(
        &ctx.text,
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
}
