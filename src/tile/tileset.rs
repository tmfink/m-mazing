use log::{debug, info, trace};
use thiserror::Error;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParsingState {
    TileName,
    WallRow { row_num: u32 },
    CellRow { row_num: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowType {
    Wall,
    Cell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Placeholder;

impl TileTokenParse for Placeholder {
    const NAME: &'static str = "Placeholder";
    const ALLOWED_CHARS: &'static str = "+";

    fn parse(value: u8) -> Option<Self> {
        if value == b'+' {
            Some(Self)
        } else {
            None
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TileParsingError {
    #[error("Expected tile name leader '@', at line {line_number} found line {line:?}")]
    InvalidNameLeader { line_number: u32, line: String },

    #[error("Finished parsing at line {line_number} before finishing tile {name:?}")]
    IncompleteTile {
        line_number: u32,
        name: Option<String>,
    },

    #[error("Expected ASCII tilename, at line {line_number} found {name:?}")]
    InvalidTileName { line_number: u32, name: String },

    #[error("Invalid number of rows at line {line_number} found {num_rows} rows")]
    WrongNumberOfRows { line_number: u32, num_rows: u32 },

    #[error("Row has extra characters on line {line_number}: {line}")]
    RowHasExtra {
        line_number: u32,
        col_number: u32,
        line: String,
    },

    #[error("Unexpected end-of-line while for line {line_number}: {line:?}")]
    IncompleteLine { line_number: u32, line: String },

    #[error("Failed to parse item {char} as {name} on line {line_number}: {line:?}; must be in {allowed}")]
    ItemParse {
        line_number: u32,
        col_number: u32,
        line: String,
        char: char,
        name: &'static str,
        allowed: &'static str,
    },
}

//pub fn tileset_from_file()

pub fn tileset_from_str(s: &str) -> Result<Vec<Tile>, TileParsingError> {
    info!("Parsing tileset");
    tileset_from_lines(s.lines())
}

#[derive(Debug, Clone)]
pub struct ParseContext<'a> {
    pub line: &'a [u8],
    pub line_number: u32,
}

fn eat_thing<C, T>(ctx: &ParseContext, cursor: &mut C) -> Result<T, TileParsingError>
where
    C: Iterator<Item = (usize, u8)>,
    T: TileTokenParse,
{
    trace!("    Eating {}", T::NAME);
    let (col_number, c) = if let Some((col_number, c)) = cursor.next() {
        (col_number, c)
    } else {
        return Err(TileParsingError::IncompleteLine {
            line_number: ctx.line_number,
            line: String::from_utf8_lossy(ctx.line).to_string(),
        });
    };

    T::parse(c).ok_or_else(|| TileParsingError::ItemParse {
        col_number: col_number as u32,
        line_number: ctx.line_number + 1,
        line: String::from_utf8_lossy(ctx.line).to_string(),
        char: char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER),
        name: T::NAME,
        allowed: T::ALLOWED_CHARS,
    })
}

pub fn tileset_from_lines<L, S>(lines: L) -> Result<Vec<Tile>, TileParsingError>
where
    L: Iterator<Item = S>,
    S: AsRef<[u8]>,
{
    // todo: escalator
    let mut line_number = 0;

    let mut tileset = Vec::new();
    let mut state = ParsingState::TileName;

    let mut tile = Tile::default();
    let mut tile_name: Option<String> = None;

    for line_x in lines {
        let line = line_x.as_ref();
        line_number += 1;

        let ctx = ParseContext { line, line_number };

        debug!(
            "line {}: {:?} ; state={:?}",
            line_number,
            String::from_utf8_lossy(line),
            state
        );

        match line.get(0) {
            None | Some(b'#') => continue,
            _ => (),
        }

        let mut cursor = line.iter().copied().enumerate();
        match state {
            ParsingState::TileName => {
                let (leader, tail) = match line {
                    &[] => unreachable!(),
                    &[leader, ref tail @ ..] => (leader, tail),
                };
                if leader != b'@' {
                    let line = String::from_utf8_lossy(line).to_string();
                    return Err(TileParsingError::InvalidNameLeader { line_number, line });
                }
                if !tail.is_ascii() {
                    return Err(TileParsingError::InvalidTileName {
                        line_number,
                        name: String::from_utf8_lossy(tail).to_string(),
                    });
                }
                tile_name = Some(String::from_utf8(tail.to_vec()).unwrap());
                tile = Tile::default();
                state = ParsingState::WallRow { row_num: 0 };

                continue;
            }
            ParsingState::WallRow { row_num } => {
                let walls = if let Some(walls) = tile.horz_walls.get_mut(row_num as usize) {
                    walls.iter_mut()
                } else {
                    return Err(TileParsingError::WrongNumberOfRows {
                        line_number,
                        num_rows: row_num + 1,
                    });
                };

                eat_thing::<_, Placeholder>(&ctx, &mut cursor)?;
                for wall in walls {
                    *wall = eat_thing(&ctx, &mut cursor)?;
                    eat_thing::<_, Placeholder>(&ctx, &mut cursor)?;
                }

                if row_num == Tile::CELL_GRID_WIDTH as u32 {
                    // done with current tile
                    debug!("Parsed tile {:#?}", tile);
                    tileset.push(tile.clone());
                    state = ParsingState::TileName;
                } else {
                    state = ParsingState::CellRow { row_num };
                }
            }
            ParsingState::CellRow { row_num } => {
                if row_num >= Tile::CELL_GRID_WIDTH as u32 {
                    return Err(TileParsingError::WrongNumberOfRows {
                        line_number,
                        num_rows: row_num + 1,
                    });
                }
                let (mut walls, cells) = if let (Some(walls), Some(cells)) = (
                    tile.vert_walls.get_mut(row_num as usize),
                    tile.cells.get_mut(row_num as usize),
                ) {
                    (walls.iter_mut(), cells.iter_mut())
                } else {
                    return Err(TileParsingError::WrongNumberOfRows {
                        line_number,
                        num_rows: row_num + 1,
                    });
                };

                *walls.next().unwrap() = eat_thing(&ctx, &mut cursor)?;
                for (cell, wall) in cells.zip(walls) {
                    *cell = eat_thing(&ctx, &mut cursor)?;
                    *wall = eat_thing(&ctx, &mut cursor)?;
                }

                state = ParsingState::WallRow {
                    row_num: row_num + 1,
                };
            }
        }

        if let Some((col_number, _c)) = cursor.next() {
            return Err(TileParsingError::RowHasExtra {
                line: String::from_utf8_lossy(line).to_string(),
                line_number,
                col_number: col_number as u32,
            });
        }
    }

    match state {
        ParsingState::TileName => Ok(tileset),
        ParsingState::CellRow { .. } | ParsingState::WallRow { .. } => {
            Err(TileParsingError::IncompleteTile {
                line_number,
                name: tile_name,
            })
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

    const TILE_SIMPLE1: &str = "
@1
+-+-+7+-+
|  1   c|
+ +-+-+ +
|   |t  |
+-+ + +-+
|   |   |
+ +-+-+ +
|O|     |
+ +^+ + +
";

    #[test]
    fn tile_simple() {
        crate::init_logging();
        let tile1 = Tile {
            cells: [
                [Empty, Warp(Green), Empty, Camera(Available)],
                [Empty, Empty, TimerFlip(Available), Empty],
                [Empty, Empty, Empty, Empty],
                [FinalExit(Orange), Empty, Empty, Empty],
            ],
            horz_walls: [
                [Blocked, Blocked, Explore(Yellow), Blocked],
                [Open, Blocked, Blocked, Open],
                [Blocked, Open, Open, Blocked],
                [Open, Blocked, Blocked, Open],
                [Open, Entrance, Open, Open],
            ],
            vert_walls: [
                [Blocked, Open, Open, Open, Blocked],
                [Blocked, Open, Blocked, Open, Blocked],
                [Blocked, Open, Blocked, Open, Blocked],
                [Blocked, Blocked, Open, Open, Blocked],
            ],
            escalators: arrayvec::ArrayVec::new(),
        };
        let expected = vec![tile1];
        assert_eq!(tileset_from_str(TILE_SIMPLE1.trim()), Ok(expected));
    }

    // todo: add negative tests
}
