use macroquad::prelude as mq;

pub fn draw_connected_line(points: &[mq::Vec2], thickness: f32, color: mq::Color) {
    for s in points.windows(2) {
        let (a, b) = match s {
            [a, b] => (a, b),
            _ => unreachable!("should only get 2 elements in window"),
        };
        mq::draw_line(a.x, a.y, b.x, b.y, thickness, color)
    }
}
