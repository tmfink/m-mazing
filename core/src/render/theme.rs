use macroquad::prelude as mq;

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg_color: mq::Color,
    pub font_color: mq::Color,
    pub font_size: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            bg_color: mq::WHITE,
            font_color: mq::DARKGRAY,
            font_size: 30.,
        }
    }
}
