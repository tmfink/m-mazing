use crate::prelude::*;
use itertools::Itertools;

pub fn draw_connected_line<I: Iterator<Item = Vec2>>(
    points: I,
    mut builder: GeometryBuilder,
) -> GeometryBuilder {
    for (a, b) in points.tuple_windows() {
        builder = builder.add(&shapes::Line(a, b));
    }
    builder
}
