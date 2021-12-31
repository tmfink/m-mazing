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

fn render_final_exit(render: &RenderState, x: f32, y: f32, pawn: Pawn) {
    let offset = 0.5 * render.theme.wall_thickness;
    mq::draw_rectangle(
        x + offset,
        y + offset,
        CELL_WIDTH - 2.0 * offset,
        CELL_WIDTH - 2.0 * offset,
        pawn.as_color(render),
    );
    // todo: add arrow pointing outward
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
            let row_idx = row_idx as f32;
            let y = -GRID_HALF_WIDTH + row_idx * CELL_WIDTH;
            for (col_idx, cell) in row.iter().copied().enumerate() {
                let col_idx = col_idx as f32;
                let x = -GRID_HALF_WIDTH + col_idx * CELL_WIDTH;

                match cell {
                    TileCell::TimerFlip(_) => render_timer(render, x, y),
                    TileCell::Warp(pawn) => render_warp(render, x, y, pawn),
                    TileCell::Loot(pawn) => render_loot(render, x, y, pawn),
                    TileCell::FinalExit(pawn) => render_final_exit(render, x, y, pawn),

                    //TileCell::Camera(_) => todo!(),
                    TileCell::Empty => (),
                    _ => (),
                }
            }
        }

        // todo: render escalators
    }
}
