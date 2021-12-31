use macroquad::prelude as mq;

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg_color: mq::Color,
    pub font_color: mq::Color,
    pub font_size: f32,
    pub tile_bg_color: mq::Color,
    pub wall_blocked_color: mq::Color,
    pub wall_open_color: mq::Color,
    pub wall_thickness: f32,
    pub warp_thickness: f32,
    pub pawn_green_color: mq::Color,
    pub pawn_orange_color: mq::Color,
    pub pawn_yellow_color: mq::Color,
    pub pawn_purple_color: mq::Color,
    pub timer_color: mq::Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            bg_color: mq::WHITE,
            font_color: mq::DARKGRAY,
            font_size: 40.,
            tile_bg_color: mq::Color {
                r: 0.76,
                g: 0.81,
                b: 0.88,
                a: 0.5,
            },
            wall_blocked_color: mq::BLACK,
            wall_open_color: mq::LIGHTGRAY,
            wall_thickness: 0.1,
            warp_thickness: 0.05,
            pawn_green_color: mq::GREEN,
            pawn_orange_color: mq::ORANGE,
            pawn_yellow_color: mq::YELLOW,
            pawn_purple_color: mq::PURPLE,
            timer_color: mq::RED,
        }
    }
}
