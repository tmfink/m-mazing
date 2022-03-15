use crate::prelude::*;

pub struct Scenario {
    pub escape: ScenarioEscape,
    pub start_tile: Tile,
    pub explore_tile_names: Vec<String>,
}

pub enum ScenarioEscape {
    /// All pawns must escape via the the Purple exit
    PurpleOnly,

    /// Each pawn must escape through its own color
    EachColor,
}
