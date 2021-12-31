use itertools::Itertools;
use macroquad::prelude as mq;

pub fn draw_connected_line<I: Iterator<Item = mq::Vec2>>(
    points: I,
    thickness: f32,
    color: mq::Color,
) {
    for (a, b) in points.tuple_windows() {
        mq::draw_line(a.x, a.y, b.x, b.y, thickness, color);
    }
}
