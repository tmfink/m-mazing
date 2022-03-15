#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CartesianDirection {
    Right,
    Up,
    Left,
    Down,
}

impl CartesianDirection {
    pub const ALL_DIRECTIONS: [CartesianDirection; 4] =
        [Self::Right, Self::Up, Self::Left, Self::Down];

    /// What to add to point to get neighbor
    pub fn neighbor_transform(self) -> (i8, i8) {
        match self {
            Self::Right => (1, 0),
            Self::Up => (0, -1),
            Self::Left => (-1, 0),
            Self::Down => (0, 1),
        }
    }

    /// CartesianDirection converted to angle (in radians)
    pub fn as_angle(self) -> f32 {
        match self {
            Self::Right => 0.,
            Self::Up => std::f32::consts::FRAC_PI_2,
            Self::Left => std::f32::consts::PI,
            Self::Down => 1.5 * std::f32::consts::PI,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpinDirection {
    Clockwise,
    CounterClockwise,
}
