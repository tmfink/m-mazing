use super::TileTokenParse;
use crate::prelude::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellItemAvailability {
    Available,
    Used,
}
