use macroquad::prelude as mq;

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg_color: mq::Color,
    pub font_color: mq::Color,
    pub font_size: f32,
    pub tile_normal_bg_color: mq::Color,
    pub tile_camera_bg_color: mq::Color,
    pub wall_blocked_color: mq::Color,
    pub wall_open_color: mq::Color,
    pub wall_orange_only_color: mq::Color,
    pub wall_orange_only_hole_width: f32,
    pub wall_entrance_color: mq::Color,
    pub wall_thickness: f32,
    pub warp_thickness: f32,
    pub loot_thickness: f32,
    pub pawn_green_color: mq::Color,
    pub pawn_orange_color: mq::Color,
    pub pawn_yellow_color: mq::Color,
    pub pawn_purple_color: mq::Color,
    pub timer_color: mq::Color,
    pub final_exit_arrow_color: mq::Color,
    pub camera_color: mq::Color,
    pub crystal_ball_color: mq::Color,
    pub escalator_color: mq::Color,
    pub escalator_thickness: f32,
    pub used_marker_thickness: f32,
    pub used_marker_color: mq::Color,
    pub unreachable_cell_color: mq::Color,
}

impl Default for Theme {
    fn default() -> Self {
        let tile_bg_color = mq::Color {
            r: 0.88,
            g: 0.90,
            b: 0.94,
            a: 1.0,
        };
        Theme {
            bg_color: mq::WHITE,
            font_color: mq::DARKGRAY,
            font_size: 40.,
            tile_normal_bg_color: tile_bg_color,
            tile_camera_bg_color: mq::Color {
                r: 0.98,
                g: 0.918,
                b: 0.8,
                a: 1.0,
            },
            wall_blocked_color: mq::BLACK,
            wall_open_color: mq::LIGHTGRAY,
            wall_orange_only_color: mq::Color {
                r: 0.8,
                g: 0.46,
                b: 0.027,
                a: 1.0,
            },
            wall_orange_only_hole_width: 0.1,
            wall_entrance_color: tile_bg_color,
            wall_thickness: 0.1,
            warp_thickness: 0.05,
            loot_thickness: 0.07,
            pawn_green_color: mq::GREEN,
            pawn_orange_color: mq::ORANGE,
            pawn_yellow_color: mq::YELLOW,
            pawn_purple_color: mq::PURPLE,
            timer_color: mq::RED,
            final_exit_arrow_color: mq::GRAY,
            camera_color: mq::BLACK,
            crystal_ball_color: mq::PURPLE,
            escalator_color: mq::Color {
                r: 0.44,
                g: 0.3,
                b: 0.39,
                a: 1.0,
            },
            escalator_thickness: 0.2,
            used_marker_thickness: 0.18,
            used_marker_color: mq::BLACK,
            unreachable_cell_color: mq::DARKGRAY,
        }
    }
}
