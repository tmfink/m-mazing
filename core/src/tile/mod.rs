use std::{collections::HashSet, str::FromStr};

use crate::prelude::*;

pub mod cell;
pub mod direction;
pub mod escalator;
pub mod grid_coord;
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
    pub fn cell_outer_edge_directions(&self, coord: TileGridCoord) -> Vec<Direction> {
        const MAX_IDX: u8 = Tile::CELL_GRID_WIDTH - 1;

        let mut dirs = Vec::with_capacity(4);
        if coord.x() == 0 {
            dirs.push(Direction::Left);
        }
        if coord.x() == MAX_IDX {
            dirs.push(Direction::Right);
        }
        if coord.y() == 0 {
            dirs.push(Direction::Up);
        }
        if coord.y() == MAX_IDX {
            dirs.push(Direction::Down);
        }
        dirs
    }

    pub fn cell_value(&self, coord: TileGridCoord) -> TileCell {
        self.cell_grid[coord.y() as usize][coord.x() as usize]
    }

    /// Neighbor coords accessible via cardinal direction walk
    pub fn cell_cardinal_neighbor_coords(
        &self,
        coord: TileGridCoord,
        direction: Direction,
    ) -> Option<TileGridCoord> {
        coord.added(direction.neighbor_transform())
    }

    /// Neighbor accessible via cardinal direction walk
    pub fn cell_cardinal_neighbor(
        &self,
        coord: TileGridCoord,
        direction: Direction,
    ) -> Option<TileCell> {
        let neighbor_point = self.cell_cardinal_neighbor_coords(coord, direction)?;
        Some(self.cell_value(neighbor_point))
    }

    /// Coordinates of "neighbors" in current tile that are "one step" away
    /// (either by cardinal direction walk or escalator).
    pub fn cell_immediate_neighbor_coords(&self, coord: TileGridCoord) -> Vec<TileGridCoord> {
        let cardinal_neighbors = Direction::ALL_DIRECTIONS.iter().copied().filter_map(|dir| {
            self.cell_cardinal_neighbor_coords(coord, dir)
                // todo: handle orange-only walls
                .filter(|_| self.cell_wall(coord, dir) == WallState::Open)
        });
        let escalator_neighbors = self
            .escalators
            .iter()
            .filter_map(|esc_loc| esc_loc.coord_neighbor(coord));

        let mut neighbors: Vec<_> = cardinal_neighbors.chain(escalator_neighbors).collect();

        // we could have duplicates
        neighbors.sort_unstable();
        neighbors.dedup();

        neighbors
    }

    const POSSIBLE_ENTRANCE_COORDS: [TileGridCoord; 4] = [
        TileGridCoord { x: 0, y: 1 },
        TileGridCoord { x: 1, y: 3 },
        TileGridCoord { x: 2, y: 0 },
        TileGridCoord { x: 3, y: 2 },
    ];

    fn reachable_coords_starting(&self) -> Vec<TileGridCoord> {
        Self::POSSIBLE_ENTRANCE_COORDS
            .iter()
            .copied()
            .filter(|coord| {
                let dir = match *self.cell_outer_edge_directions(*coord).as_slice() {
                    [dir] => dir,
                    _ => {
                        panic!("there should only be one edge direction for possible entrance node")
                    }
                };
                match self.cell_wall(*coord, dir) {
                    WallState::Entrance | WallState::Explore(_) | WallState::Open => true,
                    WallState::Blocked => false,
                }
            })
            .collect()
    }

    pub fn reachable_coords(
        &self,
    ) -> [[bool; Tile::CELL_GRID_WIDTH as usize]; Tile::CELL_GRID_WIDTH as usize] {
        let mut explore_coords = self.reachable_coords_starting();
        let mut visited_coords = HashSet::new();
        let mut is_reachable_coord: [[bool; Tile::CELL_GRID_WIDTH as usize];
            Tile::CELL_GRID_WIDTH as usize] = Default::default();

        while let Some(coord) = explore_coords.pop() {
            visited_coords.insert(coord);
            is_reachable_coord[coord.y as usize][coord.x as usize] = true;

            for neighbor in self.cell_immediate_neighbor_coords(coord) {
                if !(visited_coords.contains(&neighbor) || explore_coords.contains(&neighbor)) {
                    explore_coords.push(neighbor);
                }
            }
        }

        is_reachable_coord
    }

    pub fn cell_wall(&self, coord: TileGridCoord, direction: Direction) -> WallState {
        let x = coord.x() as usize;
        let y = coord.y() as usize;
        match direction {
            Direction::Up => self.horz_walls[y][x],
            Direction::Down => self.horz_walls[y + 1][x],
            Direction::Left => self.vert_walls[y][x],
            Direction::Right => self.vert_walls[y][x + 1],
        }
    }

    pub fn cell_exit_direction(&self, coord: TileGridCoord) -> Direction {
        let open_exit_dirs: Vec<Direction> = self
            .cell_outer_edge_directions(coord)
            .iter()
            .copied()
            .filter(|dir| self.cell_wall(coord, *dir) == WallState::Open)
            .collect();

        let dir = match open_exit_dirs.as_slice() {
            [dir1] => *dir1,
            _ => {
                warn!(
                    "Unable to find a good direction for exit direction at {:?}",
                    coord
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

    pub fn cells_iter(&self) -> impl Iterator<Item = &TileCell> {
        self.cell_grid.iter().flat_map(|row| row.iter())
    }

    pub fn cells_iter_mut(&mut self) -> impl Iterator<Item = &mut TileCell> {
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

#[cfg(test)]
mod test {
    use once_cell::sync::Lazy;

    use super::*;
    use CellItemAvailability::*;
    use Pawn::*;
    use TileCell::*;
    use WallState::*;

    static TILE_1A: Lazy<Tile> = Lazy::new(|| Tile {
        cell_grid: [
            [TimerFlip(Available), Empty, Empty, Warp(Purple)],
            [Empty, Empty, Empty, Warp(Yellow)],
            [Warp(Orange), Empty, Empty, Empty],
            [Warp(Green), Empty, Empty, Empty],
        ],
        horz_walls: [
            [Blocked, Blocked, Explore(Orange), Blocked],
            [Blocked, Open, Open, Blocked],
            [Blocked, Open, Open, Blocked],
            [Blocked, Open, Open, Blocked],
            [Blocked, Explore(Yellow), Blocked, Blocked],
        ],
        vert_walls: [
            [Blocked, Open, Open, Open, Blocked],
            [Explore(Purple), Open, Open, Open, Blocked],
            [Blocked, Open, Open, Blocked, Explore(Green)],
            [Blocked, Open, Open, Blocked, Blocked],
        ],
        escalators: [EscalatorLocation([
            TileGridCoord { x: 2, y: 3 },
            TileGridCoord { x: 3, y: 2 },
        ])]
        .iter()
        .copied()
        .collect(),
    });

    static TILE_2: Lazy<Tile> = Lazy::new(|| Tile {
        cell_grid: [
            [FinalExit(Purple), Empty, Empty, Empty],
            [Empty, Empty, Empty, Warp(Purple)],
            [Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Warp(Green)],
        ],
        horz_walls: [
            [Open, Blocked, Blocked, Blocked],
            [Open, Open, Open, Blocked],
            [Blocked, Open, Blocked, Open],
            [Open, Blocked, Open, Blocked],
            [Blocked, Explore(Orange), Blocked, Blocked],
        ],
        vert_walls: [
            [Blocked, Blocked, Open, Open, Blocked],
            [Blocked, Blocked, Open, Blocked, Blocked],
            [Blocked, Blocked, Blocked, Open, Entrance],
            [Blocked, Blocked, Open, Open, Blocked],
        ],
        escalators: [EscalatorLocation([
            TileGridCoord { x: 0, y: 1 },
            TileGridCoord { x: 1, y: 3 },
        ])]
        .iter()
        .copied()
        .collect(),
    });

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
                TileGridCoord { x: 2, y: 3 },
                TileGridCoord { x: 3, y: 2 },
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
                TileGridCoord { x: 3, y: 1 },
                TileGridCoord { x: 2, y: 0 },
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
                TileGridCoord { x: 0, y: 2 },
                TileGridCoord { x: 1, y: 3 },
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

    #[test]
    fn neighbor() {
        assert_eq!(
            TILE_1A.cell_immediate_neighbor_coords(TileGridCoord { x: 0, y: 0 }),
            [TileGridCoord { x: 1, y: 0 }]
        );
        assert_eq!(
            TILE_1A.cell_immediate_neighbor_coords(TileGridCoord { x: 1, y: 0 }),
            [
                TileGridCoord { x: 0, y: 0 },
                TileGridCoord { x: 1, y: 1 },
                TileGridCoord { x: 2, y: 0 },
            ]
        );
        assert_eq!(
            TILE_1A.cell_immediate_neighbor_coords(TileGridCoord { x: 3, y: 3 }),
            []
        );
        assert_eq!(
            TILE_1A.cell_immediate_neighbor_coords(TileGridCoord { x: 2, y: 3 }),
            [
                TileGridCoord { x: 1, y: 3 },
                TileGridCoord { x: 2, y: 2 },
                TileGridCoord { x: 3, y: 2 },
            ]
        );
        assert_eq!(
            TILE_1A.cell_immediate_neighbor_coords(TileGridCoord { x: 3, y: 2 }),
            [TileGridCoord { x: 2, y: 3 },]
        );
    }

    #[test]
    fn start_coords() {
        assert_eq!(
            TILE_1A.reachable_coords_starting(),
            Tile::POSSIBLE_ENTRANCE_COORDS
        );

        assert_eq!(
            TILE_2.reachable_coords_starting(),
            [TileGridCoord { x: 1, y: 3 }, TileGridCoord { x: 3, y: 2 },]
        );
    }

    #[test]
    fn reachable_coords() {
        assert_eq!(
            TILE_1A.reachable_coords(),
            [
                [true, true, true, true],
                [true, true, true, true],
                [true, true, true, true],
                [true, true, true, false],
            ]
        );

        assert_eq!(
            TILE_2.reachable_coords(),
            [
                [true, false, false, false],
                [true, false, false, true],
                [false, false, true, true],
                [false, true, true, true],
            ]
        );
    }
}
