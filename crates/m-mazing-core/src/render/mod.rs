pub mod camera;
pub mod render_tile;
pub mod shape;
pub mod theme;

use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct RenderState {
    pub theme: Theme,
}

pub trait Render {
    fn render(&self, pos: Vec2, render: &RenderState);
}

impl WallState {
    fn wall_color(self, render: &RenderState, tile_bg_color: Color) -> Color {
        match self {
            WallState::Explore(pawn) => pawn.as_color(render),
            WallState::Open => render.theme.wall_open_color,
            WallState::OrangeOnly => render.theme.wall_orange_only_color,
            WallState::Entrance => tile_bg_color,
            WallState::Blocked => render.theme.wall_blocked_color,
        }
    }
}

impl Pawn {
    fn as_color(self, render: &RenderState) -> Color {
        match self {
            Self::Green => render.theme.pawn_green_color,
            Self::Orange => render.theme.pawn_orange_color,
            Self::Yellow => render.theme.pawn_yellow_color,
            Self::Purple => render.theme.pawn_purple_color,
        }
    }
}

fn polar_to_cartesian(radius: f32, angle_rads: f32) -> Vec2 {
    let x = radius * angle_rads.cos();
    let y = radius * angle_rads.sin();
    Vec2::new(x, y)
}
