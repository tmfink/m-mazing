use std::str::FromStr;

use crate::Pawn;

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

    /// Blocked off
    Inaccessible,

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
}

impl Default for TileCell {
    fn default() -> Self {
        Self::Empty
    }
}

impl TileTokenParse for TileCell {
    const NAME: &'static str = "TileCell";
    const ALLOWED_CHARS: &'static str = " 1234GOYPgoypcXt";

    fn parse(value: u8) -> Option<Self> {
        Some(match value {
            b' ' => Self::Empty,
            b'X' => Self::Inaccessible,

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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct OuterWalls {
    top: [WallState; Tile::CELL_GRID_WIDTH as usize],
    left: [WallState; Tile::CELL_GRID_WIDTH as usize],
    right: [WallState; Tile::CELL_GRID_WIDTH as usize],
    bottom: [WallState; Tile::CELL_GRID_WIDTH as usize],
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tile {
    cells: [[TileCell; Self::CELL_GRID_WIDTH as usize]; Self::CELL_GRID_WIDTH as usize],
    horz_walls: [[WallState; Self::CELL_GRID_WIDTH as usize]; (Self::CELL_GRID_WIDTH + 1) as usize],
    vert_walls: [[WallState; (Self::CELL_GRID_WIDTH + 1) as usize]; Self::CELL_GRID_WIDTH as usize],
    escalators: arrayvec::ArrayVec<EscalatorLocation, { Self::MAX_ESCALATORS_PER_TILE as usize }>,
}

impl Tile {
    pub const CELL_GRID_WIDTH: u32 = 4;
    const MAX_ESCALATORS_PER_TILE: u32 = 4;
}

impl FromStr for Tile {
    type Err = tileset::TileParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tileset::tile_from_lines(&mut s.lines())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellItemAvailability {
    Available,
    Used,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TilePoint {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscalatorLocation {
    endpoints: [TilePoint; 2],
}
