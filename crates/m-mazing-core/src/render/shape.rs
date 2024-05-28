use bevy_prototype_lyon::{geometry::GeometryBuilder, shapes};
use itertools::Itertools;

use crate::prelude::*;

pub fn draw_connected_line<I: Iterator<Item = Vec2>>(
    points: I,
    mut builder: GeometryBuilder,
) -> GeometryBuilder {
    for (a, b) in points.tuple_windows() {
        builder = builder.add(&shapes::Line(a, b));
    }
    builder
}
