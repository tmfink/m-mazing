use macroquad::prelude as mq;

use crate::prelude::*;

impl Render for Tile {
    fn render(&self, pos: mq::Vec2, render: &RenderState) {
        const GRID_WIDTH: f32 = Tile::CELL_GRID_WIDTH as f32;
        const GRID_HALF_WIDTH: f32 = GRID_WIDTH / 2.0;
        const CELL_WIDTH: f32 = 1.0;

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
                    TileCell::TimerFlip(_) => {
                        let x_left = x + 0.25;
                        let x_right = x + 0.75;
                        let y_top = y + 0.2;
                        let y_bottom = y + 0.8;

                        let points = [
                            mq::Vec2::new(x_left, y_top),
                            mq::Vec2::new(x_right, y_top),
                            mq::Vec2::new(x_left, y_bottom),
                            mq::Vec2::new(x_right, y_bottom),
                            mq::Vec2::new(x_left, y_top),
                        ];
                        shape::draw_connected_line(
                            &points,
                            render.theme.wall_thickness,
                            render.theme.timer_color,
                        );
                    }

                    //TileCell::Camera(_) => todo!(),
                    //TileCell::FinalExit(_) => todo!(),
                    //TileCell::Warp(_) => todo!(),
                    //TileCell::Loot(_) => todo!(),
                    TileCell::Empty => (),
                    _ => (),
                }
            }
        }

        // todo: render escalators
    }
}
