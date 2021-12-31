pub mod camera;
pub mod shape;
pub mod theme;

use macroquad::prelude as mq;

use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct RenderState {
    pub theme: Theme,
}

pub trait Render {
    fn render(&self, pos: mq::Vec2, render: &RenderState);
}

impl WallState {
    fn wall_color(self, render: &RenderState) -> mq::Color {
        match self {
            WallState::Explore(pawn) => pawn.as_color(render),
            WallState::Open | WallState::Entrance => render.theme.wall_open_color,
            WallState::Blocked => render.theme.wall_blocked_color,
        }
    }
}

impl Pawn {
    fn as_color(self, render: &RenderState) -> mq::Color {
        match self {
            Self::Green => render.theme.pawn_green_color,
            Self::Orange => render.theme.pawn_orange_color,
            Self::Yellow => render.theme.pawn_yellow_color,
            Self::Purple => render.theme.pawn_purple_color,
        }
    }
}

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

#[derive(Clone, Copy, Debug)]
pub enum AlignVert {
    Absolute(f32),
    Top,
    Middle,
    Bottom,
}

#[derive(Clone, Copy, Debug)]
pub enum AlignHoriz {
    Absolute(f32),
    Left,
    Center,
    Right,
}

pub fn draw_text_align(text: &str, x: AlignHoriz, y: AlignVert, params: mq::TextParams) {
    let dims = mq::measure_text(text, None, params.font_size, params.font_scale);
    let x = match x {
        AlignHoriz::Absolute(x) => x,
        AlignHoriz::Left => 0.,
        AlignHoriz::Center => (mq::screen_width() - dims.width) / 2.0,
        AlignHoriz::Right => mq::screen_width() - dims.width,
    };
    let y = match y {
        AlignVert::Absolute(y) => y,
        AlignVert::Top => dims.height,
        AlignVert::Middle => (mq::screen_height() + dims.height) / 2.0,
        AlignVert::Bottom => mq::screen_height() - dims.height,
    };
    mq::draw_text_ex(text, x, y, params);
}
