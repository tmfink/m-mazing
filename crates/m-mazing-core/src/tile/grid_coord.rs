use crate::prelude::*;

/// Index (x, y) into a `Tile` grid.
///
/// - `x`: goes from left to right
/// - `y`: goes from top to bottom
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileGridCoord {
    pub(super) x: u8,
    pub(super) y: u8,
}

impl TileGridCoord {
    pub const fn new(x: u8, y: u8) -> Option<Self> {
        if x < Tile::CELL_GRID_WIDTH && y < Tile::CELL_GRID_WIDTH {
            Some(Self { x, y })
        } else {
            None
        }
    }

    pub fn added(self, add: (i8, i8)) -> Option<Self> {
        let new_x = (self.x as i8 + add.0).try_into().ok()?;
        let new_y = (self.y as i8 + add.1).try_into().ok()?;
        TileGridCoord::new(new_x, new_y)
    }

    #[inline(always)]
    pub fn x(self) -> u8 {
        self.x
    }

    #[inline(always)]
    pub fn y(self) -> u8 {
        self.y
    }

    pub fn rotate(&mut self, spin: SpinDirection) {
        *self = self.as_rotated(spin);
    }

    pub fn as_rotated(self, spin: SpinDirection) -> Self {
        match spin {
            SpinDirection::Clockwise => Self {
                x: Tile::CELL_GRID_WIDTH - 1 - self.y,
                y: self.x,
            },
            SpinDirection::CounterClockwise => Self {
                x: self.y,
                y: Tile::CELL_GRID_WIDTH - 1 - self.x,
            },
        }
    }
}
