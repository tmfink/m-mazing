use std::str::FromStr;

use crate::prelude::*;
use crate::*;

pub mod tileset;

trait TileTokenParse
where
    Self: Sized,
{
    const NAME: &'static str;
    const ALLOWED_CHARS: &'static str;
    fn parse(value: u8) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileCell {
    /// Pawn walk freely through
    Empty,

    // Pawn can be warped to this point
    Warp(Pawn),

    /// Can flip sand timer
    TimerFlip(CellItemAvailability),

    /// Security Camera
    Camera(CellItemAvailability),

    /// Loot that pawns need to "steal" before exit
    Loot(Pawn),

    /// Final exit tile
    FinalExit(Pawn),

    /// Crystal ball
    CrystalBall(CellItemAvailability),
}

impl Default for TileCell {
    fn default() -> Self {
        Self::Empty
    }
}

impl TileCell {
    pub fn is_used(self) -> bool {
        use CellItemAvailability::*;
        use TileCell::*;

        matches!(self, TimerFlip(Used) | Camera(Used) | CrystalBall(Used))
    }

    pub fn set_availability(&mut self, new_used: CellItemAvailability) {
        use TileCell::*;

        match self {
            TimerFlip(avail) | Camera(avail) | CrystalBall(avail) => {
                *avail = new_used;
            }
            _ => {}
        }
    }
}

impl TileTokenParse for TileCell {
    const NAME: &'static str = "TileCell";
    const ALLOWED_CHARS: &'static str = " 1234GOYPgoypctb";

    fn parse(value: u8) -> Option<Self> {
        Some(match value {
            b' ' => Self::Empty,

            // Warp
            b'1' => Self::Warp(Pawn::Green),
            b'2' => Self::Warp(Pawn::Orange),
            b'3' => Self::Warp(Pawn::Yellow),
            b'4' => Self::Warp(Pawn::Purple),

            // Loot
            b'g' => Self::Loot(Pawn::Green),
            b'o' => Self::Loot(Pawn::Orange),
            b'y' => Self::Loot(Pawn::Yellow),
            b'p' => Self::Loot(Pawn::Purple),

            // Explore exit
            b'G' => Self::FinalExit(Pawn::Green),
            b'O' => Self::FinalExit(Pawn::Orange),
            b'Y' => Self::FinalExit(Pawn::Yellow),
            b'P' => Self::FinalExit(Pawn::Purple),

            b't' => Self::TimerFlip(CellItemAvailability::Available),
            b'c' => Self::Camera(CellItemAvailability::Available),
            b'b' => Self::CrystalBall(CellItemAvailability::Available),

            _ => return None,
        })
    }
}

/// Whether a wall exists or not
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallState {
    Open,
    Blocked,
    Explore(Pawn),
    Entrance,
}

impl TileTokenParse for WallState {
    const ALLOWED_CHARS: &'static str = " -|^5678";
    const NAME: &'static str = "Wall";

    fn parse(value: u8) -> Option<Self> {
        Some(match value {
            b'-' | b'|' => Self::Blocked,
            b' ' => Self::Open,
            b'5' => Self::Explore(Pawn::Green),
            b'6' => Self::Explore(Pawn::Orange),
            b'7' => Self::Explore(Pawn::Yellow),
            b'8' => Self::Explore(Pawn::Purple),
            b'^' => Self::Entrance,
            _ => return None,
        })
    }
}

impl Default for WallState {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    /// What to add to point to get neighbor
    pub fn neighbor_transform(self) -> (i8, i8) {
        match self {
            Self::Right => (1, 0),
            Self::Up => (0, -1),
            Self::Left => (-1, 0),
            Self::Down => (0, 1),
        }
    }

    /// Direction converted to angle (in radians)
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tile {
    pub cells: [[TileCell; Self::CELL_GRID_WIDTH as usize]; Self::CELL_GRID_WIDTH as usize],
    pub horz_walls:
        [[WallState; Self::CELL_GRID_WIDTH as usize]; (Self::CELL_GRID_WIDTH + 1) as usize],
    pub vert_walls:
        [[WallState; (Self::CELL_GRID_WIDTH + 1) as usize]; Self::CELL_GRID_WIDTH as usize],
    pub escalators:
        arrayvec::ArrayVec<EscalatorLocation, { Self::MAX_ESCALATORS_PER_TILE as usize }>,
}

impl Tile {
    pub const CELL_GRID_WIDTH: u8 = 4;
    const MAX_ESCALATORS_PER_TILE: u8 = 4;

    /// Directions pointing to edge of tile
    ///
    /// Internal tiles will have no directions
    pub fn outer_edge_directions(&self, cell_point: TilePoint) -> Vec<Direction> {
        const MAX_IDX: u8 = Tile::CELL_GRID_WIDTH - 1;

        let mut dirs = Vec::with_capacity(4);
        if cell_point.x == 0 {
            dirs.push(Direction::Left);
        }
        if cell_point.x == MAX_IDX {
            dirs.push(Direction::Right);
        }
        if cell_point.y == 0 {
            dirs.push(Direction::Up);
        }
        if cell_point.y == MAX_IDX {
            dirs.push(Direction::Down);
        }
        dirs
    }

    pub fn cell_value(&self, point: TilePoint) -> TileCell {
        self.cells[point.y as usize][point.x as usize]
    }

    pub fn neighbor_point(&self, cell_point: TilePoint, direction: Direction) -> Option<TilePoint> {
        cell_point.added(direction.neighbor_transform())
    }

    pub fn neighbor_cell(&self, cell_point: TilePoint, direction: Direction) -> Option<TileCell> {
        let neighbor_point = self.neighbor_point(cell_point, direction)?;
        Some(self.cell_value(neighbor_point))
    }

    pub fn cell_wall(&self, cell_point: TilePoint, direction: Direction) -> WallState {
        let x = cell_point.x as usize;
        let y = cell_point.y as usize;
        match direction {
            Direction::Up => self.horz_walls[y][x],
            Direction::Down => self.horz_walls[y + 1][x],
            Direction::Left => self.vert_walls[y][x],
            Direction::Right => self.vert_walls[y][x + 1],
        }
    }

    pub fn cell_exit_direction(&self, cell_point: TilePoint) -> Direction {
        let open_exit_dirs: Vec<Direction> = self
            .outer_edge_directions(cell_point)
            .iter()
            .copied()
            .filter(|dir| self.cell_wall(cell_point, *dir) == WallState::Open)
            .collect();

        let dir = match open_exit_dirs.as_slice() {
            [dir1] => *dir1,
            _ => {
                warn!(
                    "Unable to find a good direction for exit direction at {:?}",
                    cell_point
                );
                Direction::Right
            }
        };
        dir
    }

    pub fn cells(&self) -> impl Iterator<Item = &TileCell> {
        self.cells.iter().flat_map(|row| row.iter())
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut TileCell> {
        self.cells.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn rotate(&mut self, spin: SpinDirection) {
        let mut new_tile = Self {
            escalators: self.escalators.clone(),
            ..Self::default()
        };
        rotate_2d_array(&self.cells, &mut new_tile.cells, spin);
        rotate_2d_array(&self.horz_walls, &mut new_tile.vert_walls, spin);
        rotate_2d_array(&self.vert_walls, &mut new_tile.horz_walls, spin);

        for new_esc in new_tile.escalators.iter_mut() {
            new_esc.rotate(spin);
        }
        *self = new_tile;
    }
}

pub fn rotate_2d_array<T: Copy, const WIDTH: usize, const HEIGHT: usize>(
    arr: &[[T; WIDTH]; HEIGHT],
    out: &mut [[T; HEIGHT]; WIDTH],
    spin: SpinDirection,
) {
    for (row_idx, row) in arr.iter().enumerate() {
        for (col_idx, cell) in row.iter().copied().enumerate() {
            let (a, b) = match spin {
                SpinDirection::Clockwise => (col_idx, HEIGHT - 1 - row_idx),
                SpinDirection::CounterClockwise => (WIDTH - 1 - col_idx, row_idx),
            };
            out[a][b] = cell;
        }
    }
}

impl FromStr for Tile {
    type Err = tileset::TileParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tileset::tile_from_str(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellItemAvailability {
    Available,
    Used,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilePoint {
    x: u8,
    y: u8,
}

impl TilePoint {
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
        TilePoint::new(new_x, new_y)
    }

    pub fn x(self) -> u8 {
        self.x
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscalatorLocation(pub [TilePoint; 2]);

impl EscalatorLocation {
    pub fn rotate(&mut self, spin: SpinDirection) {
        for point in self.0.iter_mut() {
            point.rotate(spin);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use CellItemAvailability::*;
    use Pawn::*;
    use TileCell::*;
    use WallState::*;

    #[test]
    fn rotate() {
        let tile = Tile {
            cells: [
                [
                    Loot(Yellow),
                    Camera(Available),
                    FinalExit(Purple),
                    Loot(Purple),
                ],
                [FinalExit(Yellow), Warp(Green), Warp(Orange), Loot(Green)],
                [
                    TimerFlip(Available),
                    Warp(Yellow),
                    Warp(Purple),
                    FinalExit(Green),
                ],
                [
                    Loot(Orange),
                    FinalExit(Orange),
                    CrystalBall(Available),
                    Empty,
                ],
            ],
            horz_walls: [
                [Blocked, Blocked, Open, Blocked],
                [Blocked, Open, Open, Blocked],
                [Blocked, Open, Open, Blocked],
                [Blocked, Open, Open, Blocked],
                [Blocked, Open, Blocked, Blocked],
            ],
            vert_walls: [
                [Blocked, Open, Open, Open, Blocked],
                [Open, Open, Open, Open, Blocked],
                [Blocked, Open, Open, Blocked, Open],
                [Blocked, Open, Open, Blocked, Blocked],
            ],
            escalators: [EscalatorLocation([
                TilePoint { x: 2, y: 3 },
                TilePoint { x: 3, y: 2 },
            ])]
            .iter()
            .copied()
            .collect(),
        };

        let tile_counterclockwise = Tile {
            cells: [
                [Loot(Purple), Loot(Green), FinalExit(Green), Empty],
                [
                    FinalExit(Purple),
                    Warp(Orange),
                    Warp(Purple),
                    CrystalBall(Available),
                ],
                [
                    Camera(Available),
                    Warp(Green),
                    Warp(Yellow),
                    FinalExit(Orange),
                ],
                [
                    Loot(Yellow),
                    FinalExit(Yellow),
                    TimerFlip(Available),
                    Loot(Orange),
                ],
            ],
            horz_walls: [
                [Blocked, Blocked, Open, Blocked],
                [Open, Open, Blocked, Blocked],
                [Open, Open, Open, Open],
                [Open, Open, Open, Open],
                [Blocked, Open, Blocked, Blocked],
            ],
            vert_walls: [
                [Blocked, Blocked, Blocked, Blocked, Blocked],
                [Open, Open, Open, Open, Blocked],
                [Blocked, Open, Open, Open, Open],
                [Blocked, Blocked, Blocked, Blocked, Blocked],
            ],
            escalators: [EscalatorLocation([
                TilePoint { x: 3, y: 1 },
                TilePoint { x: 2, y: 0 },
            ])]
            .iter()
            .copied()
            .collect(),
        };

        let tile_clockwise = Tile {
            cells: [
                [
                    Loot(Orange),
                    TimerFlip(Available),
                    FinalExit(Yellow),
                    Loot(Yellow),
                ],
                [
                    FinalExit(Orange),
                    Warp(Yellow),
                    Warp(Green),
                    Camera(Available),
                ],
                [
                    CrystalBall(Available),
                    Warp(Purple),
                    Warp(Orange),
                    FinalExit(Purple),
                ],
                [Empty, FinalExit(Green), Loot(Green), Loot(Purple)],
            ],
            horz_walls: [
                [Blocked, Blocked, Open, Blocked],
                [Open, Open, Open, Open],
                [Open, Open, Open, Open],
                [Blocked, Blocked, Open, Open],
                [Blocked, Open, Blocked, Blocked],
            ],
            vert_walls: [
                [Blocked, Blocked, Blocked, Blocked, Blocked],
                [Open, Open, Open, Open, Blocked],
                [Blocked, Open, Open, Open, Open],
                [Blocked, Blocked, Blocked, Blocked, Blocked],
            ],
            escalators: [EscalatorLocation([
                TilePoint { x: 0, y: 2 },
                TilePoint { x: 1, y: 3 },
            ])]
            .iter()
            .copied()
            .collect(),
        };

        {
            let mut actual_counterclockwise = tile.clone();
            actual_counterclockwise.rotate(SpinDirection::CounterClockwise);
            assert_eq!(actual_counterclockwise, tile_counterclockwise);
        }

        {
            let mut actual_clockwise = tile.clone();
            actual_clockwise.rotate(SpinDirection::Clockwise);
            assert_eq!(actual_clockwise, tile_clockwise);
        }

        {
            let mut revert1 = tile.clone();
            revert1.rotate(SpinDirection::Clockwise);
            revert1.rotate(SpinDirection::CounterClockwise);
            assert_eq!(revert1, tile);

            revert1.rotate(SpinDirection::CounterClockwise);
            revert1.rotate(SpinDirection::Clockwise);
            assert_eq!(revert1, tile);

            revert1.rotate(SpinDirection::CounterClockwise);
            revert1.rotate(SpinDirection::CounterClockwise);
            revert1.rotate(SpinDirection::CounterClockwise);
            revert1.rotate(SpinDirection::CounterClockwise);
            assert_eq!(revert1, tile);

            revert1.rotate(SpinDirection::Clockwise);
            revert1.rotate(SpinDirection::Clockwise);
            revert1.rotate(SpinDirection::Clockwise);
            revert1.rotate(SpinDirection::Clockwise);
            assert_eq!(revert1, tile);
        }
    }
}
