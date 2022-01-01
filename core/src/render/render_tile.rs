use macroquad::prelude as mq;

use crate::prelude::*;

const GRID_WIDTH: f32 = Tile::CELL_GRID_WIDTH as f32;
const GRID_HALF_WIDTH: f32 = 0.5 * GRID_WIDTH;
const CELL_WIDTH: f32 = 1.0;
const CELL_HALF_WIDTH: f32 = 0.5 * CELL_WIDTH;

fn render_timer(render: &RenderState, x: f32, y: f32) {
    let x_left = x + 0.25 * CELL_WIDTH;
    let x_right = x + 0.75 * CELL_WIDTH;
    let y_top = y + 0.2 * CELL_WIDTH;
    let y_bottom = y + 0.8 * CELL_WIDTH;

    let points = [
        mq::Vec2::new(x_left, y_top),
        mq::Vec2::new(x_right, y_top),
        mq::Vec2::new(x_left, y_bottom),
        mq::Vec2::new(x_right, y_bottom),
        mq::Vec2::new(x_left, y_top),
    ];
    shape::draw_connected_line(
        points.iter().copied(),
        render.theme.wall_thickness,
        render.theme.timer_color,
    );
}

fn render_warp(render: &RenderState, x: f32, y: f32, pawn: Pawn) {
    let center = mq::Vec2::new(x + CELL_HALF_WIDTH, y + CELL_HALF_WIDTH);

    const NUM_ANGLES: u32 = 8;
    const NUM_RADII: u32 = 24;

    let angles = (0..NUM_ANGLES)
        .map(|x| x as f32 * 2.0 * std::f32::consts::PI / NUM_ANGLES as f32)
        .cycle();
    let radii = (0..NUM_RADII).map(|x| x as f32 * CELL_HALF_WIDTH * 0.8 / NUM_RADII as f32);
    let points = angles
        .zip(radii)
        .map(|(angle, radius)| mq::polar_to_cartesian(radius, angle) + center);
    shape::draw_connected_line(points, render.theme.warp_thickness, pawn.as_color(render));
}

fn render_loot(render: &RenderState, x: f32, y: f32, pawn: Pawn) {
    let x_left = x + 0.1 * CELL_WIDTH;
    let x_mid = x + 0.5 * CELL_WIDTH;
    let x_right = x + 0.9 * CELL_WIDTH;
    let y_top = y + 0.1 * CELL_WIDTH;
    let y_mid = y + 0.5 * CELL_WIDTH;
    let y_bottom = y + 0.9 * CELL_WIDTH;

    let points = [
        mq::Vec2::new(x_right, y_mid),
        mq::Vec2::new(x_mid, y_top),
        mq::Vec2::new(x_left, y_mid),
        mq::Vec2::new(x_mid, y_bottom),
        mq::Vec2::new(x_right, y_mid),
    ];
    shape::draw_connected_line(
        points.iter().copied(),
        render.theme.wall_thickness,
        pawn.as_color(render),
    );
}

fn render_camera(render: &RenderState, x: f32, y: f32) {
    let gl = unsafe { mq::get_internal_gl().quad_gl };

    let scale = mq::Vec3::new(CELL_WIDTH, CELL_WIDTH, 1.);
    let translation = mq::Vec3::new(x + 0.5, y + 0.5, 0.);
    let rotation = mq::Quat::IDENTITY;
    gl.push_model_matrix(mq::Mat4::from_scale_rotation_translation(
        scale,
        rotation,
        translation,
    ));

    let points = [
        mq::Vec2::new(-0.35, 0.0),
        mq::Vec2::new(-0.175, 0.15),
        mq::Vec2::new(0.0, 0.2),
        mq::Vec2::new(0.175, 0.15),
        mq::Vec2::new(0.35, 0.0),
    ];

    let color = render.theme.camera_color;
    shape::draw_connected_line(points.iter().copied(), render.theme.warp_thickness, color);
    shape::draw_connected_line(
        points.iter().copied().map(|mut v| {
            v.y *= -1.0;
            v
        }),
        render.theme.warp_thickness,
        color,
    );
    mq::draw_circle(0.0, 0.0, 0.15, color);

    gl.pop_model_matrix();
}

fn render_final_exit(
    render: &RenderState,
    x: f32,
    y: f32,
    pawn: Pawn,
    tile: &Tile,
    col_idx: usize,
    row_idx: usize,
) {
    let gl = unsafe { mq::get_internal_gl().quad_gl };

    let scale = mq::Vec3::new(CELL_WIDTH, CELL_WIDTH, 1.);

    let point = TilePoint::new(col_idx as u8, row_idx as u8)
        .expect("could not convert row/col idx to tile");

    // Z-axis goes "into" the screen since this is right-handed
    let angle = -tile.cell_exit_direction(point).as_angle();
    let rotation = mq::Quat::from_rotation_z(angle);

    let translation = mq::Vec3::new(x + 0.5, y + 0.5, 0.);
    gl.push_model_matrix(mq::Mat4::from_scale_rotation_translation(
        scale,
        rotation,
        translation,
    ));

    let offset = 0.5 * render.theme.wall_thickness;
    let end = 1.0 - 2.0 * offset;
    mq::draw_rectangle(
        -0.5 + offset,
        -0.5 + offset,
        end,
        end,
        pawn.as_color(render),
    );

    let color = render.theme.final_exit_arrow_color;
    let thickness = render.theme.warp_thickness;
    let endpoint = 0.5 - 2.0 * offset;
    let arrowhead_width = 0.2;
    let arrowhead_halfwidth = 0.5 * arrowhead_width;
    let arrowhead_length = 0.25;
    let arrowhead_back_x = endpoint - arrowhead_length;
    mq::draw_line(0.0, 0.0, endpoint, 0.0, thickness, color);
    mq::draw_line(
        endpoint,
        0.0,
        arrowhead_back_x,
        -arrowhead_halfwidth,
        thickness,
        color,
    );
    mq::draw_line(
        endpoint,
        0.0,
        arrowhead_back_x,
        arrowhead_halfwidth,
        thickness,
        color,
    );

    gl.pop_model_matrix();
}

impl Render for Tile {
    fn render(&self, pos: mq::Vec2, render: &RenderState) {
        mq::draw_rectangle(
            pos.x + -GRID_HALF_WIDTH,
            pos.y + -GRID_HALF_WIDTH,
            GRID_WIDTH,
            GRID_WIDTH,
            render.theme.tile_bg_color,
        );

        // horizontal walls
        for (row_idx, row) in self.horz_walls.iter().enumerate() {
            let row_idx = row_idx as f32;
            let y = -GRID_HALF_WIDTH + row_idx * CELL_WIDTH;
            for (col_idx, wall) in row.iter().copied().enumerate() {
                let col_idx = col_idx as f32;
                let x = -GRID_HALF_WIDTH + col_idx * CELL_WIDTH;
                mq::draw_line(
                    x,
                    y,
                    x + CELL_WIDTH,
                    y,
                    render.theme.wall_thickness,
                    wall.wall_color(render),
                );
            }
        }

        // vertical walls
        for (row_idx, row) in self.vert_walls.iter().enumerate() {
            let row_idx = row_idx as f32;
            let y = -GRID_HALF_WIDTH + row_idx * CELL_WIDTH;
            for (col_idx, wall) in row.iter().copied().enumerate() {
                let col_idx = col_idx as f32;
                let x = -GRID_HALF_WIDTH + col_idx * CELL_WIDTH;
                mq::draw_line(
                    x,
                    y,
                    x,
                    y + CELL_WIDTH,
                    render.theme.wall_thickness,
                    wall.wall_color(render),
                );
            }
        }

        // todo: render cells
        for (row_idx, row) in self.cells.iter().enumerate() {
            let row_idx_float = row_idx as f32;
            let y = -GRID_HALF_WIDTH + row_idx_float * CELL_WIDTH;
            for (col_idx, cell) in row.iter().copied().enumerate() {
                let col_idx_float = col_idx as f32;
                let x = -GRID_HALF_WIDTH + col_idx_float * CELL_WIDTH;

                match cell {
                    TileCell::TimerFlip(_) => render_timer(render, x, y),
                    TileCell::Warp(pawn) => render_warp(render, x, y, pawn),
                    TileCell::Loot(pawn) => render_loot(render, x, y, pawn),
                    TileCell::FinalExit(pawn) => {
                        render_final_exit(render, x, y, pawn, self, col_idx, row_idx)
                    }
                    TileCell::Camera(_) => render_camera(render, x, y),
                    TileCell::Empty => (),
                    _ => (),
                }
            }
        }

        // todo: render escalators
    }
}
