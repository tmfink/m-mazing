use crate::*;

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg_color: Color,
    pub font_color: Color,
    pub font_size: f32,
    pub tile_normal_bg_color: Color,
    pub tile_camera_bg_color: Color,
    pub wall_blocked_color: Color,
    pub wall_open_color: Color,
    pub wall_orange_only_color: Color,
    pub wall_orange_only_hole_width: f32,
    pub wall_entrance_color: Color,
    pub wall_thickness: f32,
    pub warp_thickness: f32,
    pub loot_thickness: f32,
    pub pawn_green_color: Color,
    pub pawn_orange_color: Color,
    pub pawn_yellow_color: Color,
    pub pawn_purple_color: Color,
    pub timer_color: Color,
    pub final_exit_arrow_color: Color,
    pub camera_color: Color,
    pub crystal_ball_color: Color,
    pub escalator_color: Color,
    pub escalator_thickness: f32,
    pub used_marker_thickness: f32,
    pub used_marker_color: Color,
    pub unreachable_cell_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        let tile_bg_color = Color::rgb(0.88, 0.90, 0.94);
        Theme {
            bg_color: Color::WHITE,
            font_color: Color::DARK_GRAY,
            font_size: 40.,
            tile_normal_bg_color: tile_bg_color,
            tile_camera_bg_color: Color::rgb(0.98, 0.918, 0.8),
            wall_blocked_color: Color::BLACK,
            wall_open_color: Color::rgb(0.77, 0.77, 0.77),
            wall_orange_only_color: Color::rgb(0.8, 0.46, 0.027),
            wall_orange_only_hole_width: 0.1,
            wall_entrance_color: tile_bg_color,
            wall_thickness: 0.1,
            warp_thickness: 0.05,
            loot_thickness: 0.07,
            pawn_green_color: Color::GREEN,
            pawn_orange_color: Color::ORANGE,
            pawn_yellow_color: Color::YELLOW,
            pawn_purple_color: Color::PURPLE,
            timer_color: Color::RED,
            final_exit_arrow_color: Color::GRAY,
            camera_color: Color::BLACK,
            crystal_ball_color: Color::PURPLE,
            escalator_color: Color::rgb(0.44, 0.3, 0.39),
            escalator_thickness: 0.2,
            used_marker_thickness: 0.18,
            used_marker_color: Color::BLACK,
            unreachable_cell_color: Color::DARK_GRAY,
        }
    }
}
