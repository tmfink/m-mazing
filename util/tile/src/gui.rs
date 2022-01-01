use crate::*;

const LEGEND: &str = "
Arrow keys - cycle between tiles;
K/U - toggle cell availability
";

pub enum Continuation {
    Continue,
    Exit,
}

#[must_use]
pub fn update(ctx: &mut Ctx, render: &RenderState) -> Continuation {
    if mq::is_key_pressed(mq::KeyCode::Q) || mq::is_key_pressed(mq::KeyCode::Escape) {
        return Continuation::Exit;
    }

    if mq::is_key_pressed(mq::KeyCode::Right) || mq::is_key_pressed(mq::KeyCode::Down) {
        ctx.tile_idx += 1;
    }
    if mq::is_key_pressed(mq::KeyCode::Left) || mq::is_key_pressed(mq::KeyCode::Up) {
        ctx.tile_idx -= 1;
    }
    ctx.tile_idx = ctx.tile_idx.rem_euclid(ctx.tileset.len() as isize);
    let tile = ctx.tileset.get_mut(ctx.tile_idx as usize);

    if mq::is_key_pressed(mq::KeyCode::K) || mq::is_key_pressed(mq::KeyCode::U) {
        ctx.availability = match ctx.availability {
            CellItemAvailability::Available => CellItemAvailability::Used,
            CellItemAvailability::Used => CellItemAvailability::Available,
        };
    }

    if mq::is_key_pressed(mq::KeyCode::LeftBracket) {
        //tile.rotate();
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

    ctx.text = if let Some((tile_name, tile)) = tile {
        for cell in tile.cells_mut() {
            cell.set_availability(ctx.availability);
        }
        tile.render(mq::Vec2::default(), render);
        format!(
            "TILE: {} (idx={}, avail={:?})",
            tile_name, ctx.tile_idx, ctx.availability
        )
    } else {
        "no tile".to_string()
    };

    Continuation::Continue
}

pub fn draw(ctx: &Ctx, render: &RenderState) {
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
