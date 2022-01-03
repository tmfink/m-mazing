use std::str::FromStr;

use crate::prelude::*;

pub mod cell;
pub mod direction;
pub mod grid_index;
pub mod tileset;
pub mod wall;

trait TileTokenParse
where
    Self: Sized,
{
    const NAME: &'static str;
    const ALLOWED_CHARS: &'static str;
    fn parse(value: u8) -> Option<Self>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tile {
    cell_grid: [[TileCell; Self::CELL_GRID_WIDTH as usize]; Self::CELL_GRID_WIDTH as usize],
    horz_walls: [[WallState; Self::CELL_GRID_WIDTH as usize]; (Self::CELL_GRID_WIDTH + 1) as usize],
    vert_walls: [[WallState; (Self::CELL_GRID_WIDTH + 1) as usize]; Self::CELL_GRID_WIDTH as usize],
    escalators: arrayvec::ArrayVec<EscalatorLocation, { Self::MAX_ESCALATORS_PER_TILE as usize }>,
}

impl Tile {
    pub const CELL_GRID_WIDTH: u8 = 4;
    const MAX_ESCALATORS_PER_TILE: u8 = 4;

    /// Directions pointing to edge of tile
    ///
    /// Internal tiles will have no directions
    pub fn outer_edge_directions(&self, cell_point: TileGridIdx) -> Vec<Direction> {
        const MAX_IDX: u8 = Tile::CELL_GRID_WIDTH - 1;

        let mut dirs = Vec::with_capacity(4);
        if cell_point.x() == 0 {
            dirs.push(Direction::Left);
        }
        if cell_point.x() == MAX_IDX {
            dirs.push(Direction::Right);
        }
        if cell_point.y() == 0 {
            dirs.push(Direction::Up);
        }
        if cell_point.y() == MAX_IDX {
            dirs.push(Direction::Down);
        }
        dirs
    }

    pub fn cell_value(&self, point: TileGridIdx) -> TileCell {
        self.cell_grid[point.y() as usize][point.x() as usize]
    }

    pub fn neighbor_point(
        &self,
        cell_point: TileGridIdx,
        direction: Direction,
    ) -> Option<TileGridIdx> {
        cell_point.added(direction.neighbor_transform())
    }

    pub fn neighbor_cell(&self, cell_point: TileGridIdx, direction: Direction) -> Option<TileCell> {
        let neighbor_point = self.neighbor_point(cell_point, direction)?;
        Some(self.cell_value(neighbor_point))
    }

    pub fn cell_wall(&self, cell_point: TileGridIdx, direction: Direction) -> WallState {
        let x = cell_point.x() as usize;
        let y = cell_point.y() as usize;
        match direction {
            Direction::Up => self.horz_walls[y][x],
            Direction::Down => self.horz_walls[y + 1][x],
            Direction::Left => self.vert_walls[y][x],
            Direction::Right => self.vert_walls[y][x + 1],
        }
    }

    pub fn cell_exit_direction(&self, cell_point: TileGridIdx) -> Direction {
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

    pub fn cell_grid(
        &self,
    ) -> &[[TileCell; Self::CELL_GRID_WIDTH as usize]; Self::CELL_GRID_WIDTH as usize] {
        &self.cell_grid
    }

    pub fn horz_walls(
        &self,
    ) -> &[[WallState; Self::CELL_GRID_WIDTH as usize]; (Self::CELL_GRID_WIDTH + 1) as usize] {
        &self.horz_walls
    }

    pub fn vert_walls(
        &self,
    ) -> &[[WallState; (Self::CELL_GRID_WIDTH + 1) as usize]; Self::CELL_GRID_WIDTH as usize] {
        &self.vert_walls
    }

    pub fn escalators(
        &self,
    ) -> &arrayvec::ArrayVec<EscalatorLocation, { Self::MAX_ESCALATORS_PER_TILE as usize }> {
        &self.escalators
    }

    pub fn cells(&self) -> impl Iterator<Item = &TileCell> {
        self.cell_grid.iter().flat_map(|row| row.iter())
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut TileCell> {
        self.cell_grid.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn rotate(&mut self, spin: SpinDirection) {
        let mut new_tile = Self {
            escalators: self.escalators.clone(),
            ..Self::default()
        };
        rotate_2d_array(&self.cell_grid, &mut new_tile.cell_grid, spin);
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
pub struct EscalatorLocation(pub [TileGridIdx; 2]);

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
            cell_grid: [
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
                TileGridIdx { x: 2, y: 3 },
                TileGridIdx { x: 3, y: 2 },
            ])]
            .iter()
            .copied()
            .collect(),
        };

        let tile_counterclockwise = Tile {
            cell_grid: [
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
                TileGridIdx { x: 3, y: 1 },
                TileGridIdx { x: 2, y: 0 },
            ])]
            .iter()
            .copied()
            .collect(),
        };

        let tile_clockwise = Tile {
            cell_grid: [
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
                TileGridIdx { x: 0, y: 2 },
                TileGridIdx { x: 1, y: 3 },
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
