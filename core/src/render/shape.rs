use bevy::prelude::*;
use itertools::Itertools;

pub fn draw_connected_line<I: Iterator<Item = Vec2>>(points: I, thickness: f32, color: Color) {
    for (a, b) in points.tuple_windows() {
        todo!("draw line")
        //draw_line(a.x, a.y, b.x, b.y, thickness, color);
    }
}
