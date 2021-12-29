pub mod theme;

use macroquad::prelude as mq;

#[derive(Clone, Debug, Default)]
pub struct RenderState {
    pub theme: theme::Theme,
}

pub trait Render {
    fn render();
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
