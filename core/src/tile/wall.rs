use super::TileTokenParse;
use crate::prelude::*;

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
