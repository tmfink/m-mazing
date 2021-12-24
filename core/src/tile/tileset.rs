use std::fmt::Display;

use log::{debug, info, trace};
use thiserror::Error;

use super::*;

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

    #[error("Incomple tile at line {line_number}")]
    IncompleteTile { line_number: u32 },

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

    #[error("Invalid escalator specification {0:?}")]
    InvalidEscalator(#[from] InvalidEscalator),

    #[error("No more tiles found")]
    NoMoreTiles,
}
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub struct InvalidEscalator {
    line_number: u32,
    line: String,
    msg: &'static str,
}

impl Display for InvalidEscalator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "on line {}: {:?}; {}",
            self.line_number, self.line, self.msg
        )
    }
}

impl InvalidEscalator {
    fn with_msg(&self, msg: &'static str) -> Self {
        Self {
            msg,
            ..self.clone()
        }
    }
}

pub fn tileset_from_str(s: &str) -> Result<Vec<(String, Tile)>, TileParsingError> {
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

fn tileset_from_lines<L, S>(lines: L) -> Result<Vec<(String, Tile)>, TileParsingError>
where
    L: Iterator<Item = S>,
    S: AsRef<[u8]>,
{
    let mut lines = lines.enumerate().map(|(idx, line)| (idx + 1, line));
    let mut tileset = Vec::new();

    loop {
        let line = lines.next();
        let (line_number, line) = match &line {
            Some((line_number, line)) => (*line_number as u32, line.as_ref()),
            None => break,
        };

        debug!(
            "tileset line {}: {:?}",
            line_number,
            String::from_utf8_lossy(line),
        );

        if skip_tile_line(line) {
            continue;
        }

        let (leader, tail) = match *line {
            [] => unreachable!(),
            [leader, ref tail @ ..] => (leader, tail),
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
        // todo: save tile name
        let tile_name = String::from_utf8(tail.to_vec()).unwrap();
        debug!("parsed tile_name {:?}", tile_name);

        let tile = tile_from_lines(&mut lines)?;
        tileset.push((tile_name, tile));
    }

    Ok(tileset)
}

pub(super) fn tile_from_str(s: &str) -> Result<Tile, TileParsingError> {
    let mut lines = s.lines().enumerate().map(|(idx, line)| (idx + 1, line));
    tile_from_lines(&mut lines)
}

fn skip_tile_line(line: &[u8]) -> bool {
    matches!(line.get(0), None | Some(b'#'))
}

/// Creates a `Tile` from an iterator that produces (line_number, line).
/// *line_number* is 1-based indexing.
fn tile_from_lines<L, S>(lines: &mut L) -> Result<Tile, TileParsingError>
where
    L: Iterator<Item = (usize, S)>,
    S: AsRef<[u8]>,
{
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ParsingState {
        WallRow { row_num: u32 },
        CellRow { row_num: u32 },
        Elevator,
    }
    let mut allow_line_skips = true;

    // todo: escalator
    let mut line_number = 0;

    let mut state = ParsingState::WallRow { row_num: 0 };

    let mut tile = Tile::default();

    for (line_number_x, line_x) in lines {
        line_number = line_number_x as u32;
        let line = line_x.as_ref();

        let ctx = ParseContext { line, line_number };

        debug!(
            "line {}: {:?} ; state={:?}; allow_line_skips={:?}",
            line_number,
            String::from_utf8_lossy(line),
            state,
            allow_line_skips,
        );

        if allow_line_skips && skip_tile_line(line) {
            continue;
        }

        let mut cursor = line.iter().copied().enumerate();
        match state {
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
                    state = ParsingState::Elevator;
                    allow_line_skips = false;
                    continue;
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
            ParsingState::Elevator => {
                elevators_from_line(&ctx, line, &mut tile)?;
                break;
            }
        }

        // Ensure row does not have extra
        if let Some((col_number, _c)) = cursor.next() {
            return Err(TileParsingError::RowHasExtra {
                line: String::from_utf8_lossy(line).to_string(),
                line_number,
                col_number: col_number as u32,
            });
        }
    }
    match state {
        ParsingState::CellRow { .. } | ParsingState::WallRow { .. } => {
            Err(TileParsingError::IncompleteTile { line_number })
        }
        ParsingState::Elevator => {
            debug!("Parsed tile {:#?}", tile);
            Ok(tile)
        }
    }
}

fn elevators_from_line(
    ctx: &ParseContext,
    line: &[u8],
    tile: &mut Tile,
) -> Result<(), InvalidEscalator> {
    trace!("    parsing elevators");
    let err = InvalidEscalator {
        line_number: ctx.line_number,
        line: String::from_utf8_lossy(line).to_string(),
        msg: "",
    };
    let rest = match line {
        [b'E', b':', rest @ ..] => {
            trace!("    found elevator prefix");
            rest
        }
        [] => {
            trace!("    found empty elevator line");
            return Ok(());
        }
        _ => {
            return Err(InvalidEscalator {
                msg: "Invalid prefix",
                ..err
            });
        }
    };
    let escalator_hunks = std::str::from_utf8(rest)
        .map_err(|_| InvalidEscalator {
            msg: "Invalid UTF-8",
            ..err.clone()
        })?
        .split(',');
    for hunk in escalator_hunks {
        if let [a, b, b'-', c, d] = hunk.trim().as_bytes() {
            let byte_to_digit = |c: u8| -> Result<u8, InvalidEscalator> {
                let c = char::from_u32(c as u32).expect("Invalid UTF-8 byte after conversion");
                let digit = c.to_digit(10).ok_or_else(|| InvalidEscalator {
                    msg: "Unable to parse digit",
                    ..err.clone()
                })?;
                let digit_byte: u8 = digit
                    .try_into()
                    .expect("Failed to convert decimal digit to u8");
                Ok(digit_byte)
            };
            let a = byte_to_digit(*a)?;
            let b = byte_to_digit(*b)?;
            let c = byte_to_digit(*c)?;
            let d = byte_to_digit(*d)?;

            let point1 = TilePoint::new(a, b)
                .ok_or_else(|| err.clone().with_msg("Invalid tile coordinates"))?;
            let point2 =
                TilePoint::new(c, d).ok_or_else(|| err.with_msg("Invalid tile coordinates"))?;

            let escalator = EscalatorLocation([point1, point2]);
            tile.escalators
                .try_push(escalator)
                .map_err(|_| err.clone().with_msg("Exceeded max escalators"))?;
        } else {
            return Err(err.with_msg("invalid escalator hunk"));
        };
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use once_cell::sync::Lazy;

    use super::*;
    use CellItemAvailability::*;
    use Pawn::*;
    use TileCell::*;
    use WallState::*;

    const TILE1_STR: &str = "
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
    const TILE1: Tile = Tile {
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
        escalators: arrayvec::ArrayVec::new_const(),
    };

    const TILE2_STR: &str = "
+-+-+6+-+
|t     4|
+-+ + +-+
8      3|
+-+ + +-+
|2    | 5
+-+ + +-+
|1    | |
+-+7+-+-+
";
    const TILE2: Tile = Tile {
        cells: [
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
        escalators: arrayvec::ArrayVec::new_const(),
    };

    const TILE3_STR: &str = "
+-+-+6+-+
|t     4|
+-+ + +-+
8      3|
+-+ + +-+
|2    | 5
+-+ + +-+
|1    | |
+-+7+-+-+
E: 01-23, 00-33, 33-02
";

    static TILE3: Lazy<Tile> = Lazy::new(|| Tile {
        cells: [
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
        escalators: [
            EscalatorLocation([TilePoint { x: 0, y: 1 }, TilePoint { x: 2, y: 3 }]),
            EscalatorLocation([TilePoint { x: 0, y: 0 }, TilePoint { x: 3, y: 3 }]),
            EscalatorLocation([TilePoint { x: 3, y: 3 }, TilePoint { x: 0, y: 2 }]),
        ]
        .iter()
        .copied()
        .collect(),
    });

    const TILE_MISSING_NAME: &str = "
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

        assert_eq!(TILE1_STR.parse::<Tile>(), Ok(TILE1));
        assert_eq!(TILE2_STR.parse::<Tile>(), Ok(TILE2));

        let tileset_1 = format!("@tile1\n{}", TILE1_STR);
        assert_eq!(
            tileset_from_str(&tileset_1),
            Ok(vec![("tile1".to_string(), TILE1.clone()),])
        );

        let tileset_12 = format!("@tile1\n{}\n@tile2\n{}", TILE1_STR, TILE2_STR);
        assert_eq!(
            tileset_from_str(&tileset_12),
            Ok(vec![
                ("tile1".to_string(), TILE1.clone()),
                ("tile2".to_string(), TILE2.clone()),
            ]),
        );

        let tileset_31 = format!("@tile3\n{}\n@tile1\n{}", TILE3_STR, TILE1_STR);
        assert_eq!(
            tileset_from_str(&tileset_31),
            Ok(vec![
                ("tile3".to_string(), TILE3.clone()),
                ("tile1".to_string(), TILE1.clone()),
            ]),
        );
    }

    #[test]
    fn tile_negative() {
        crate::init_logging();
        assert_eq!(tileset_from_str(""), Ok(vec![]));
        let actual = tileset_from_str(TILE_MISSING_NAME);
        assert!(
            matches!(actual, Err(TileParsingError::InvalidNameLeader { .. })),
            "actual = {:?}",
            actual
        );
    }

    #[test]
    fn fail_elevators() {
        crate::init_logging();

        let elevator_lines = [
            "foo:",
            "E::",
            "E: 01-23, 00-33z",
            "E: 01-23, 00-34",
            "E: 01-23, 00-3s",
        ];
        for el in elevator_lines {
            let tile_str = format!("{}\n{}", TILE1_STR.trim(), el);
            let actual = tile_str.parse::<Tile>();
            assert!(
                matches!(actual, Err(TileParsingError::InvalidEscalator { .. })),
                "Did not get expected Err for tile_str: {}; actual={:?}",
                tile_str,
                actual
            );
        }
    }
}
