pub mod action;
pub mod role;
pub mod tile;

#[cfg(feature = "gui")]
pub mod render;

#[cfg(feature = "gui")]
pub use macroquad;

pub struct PlayerId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pawn {
    Green,
    Orange,
    Yellow,
    Purple,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub num_players: u8,
    _roles: &'static [&'static [action::BoardAction]],
}

impl GameState {
    pub fn new(num_players: u8) -> Option<Self> {
        let _roles = crate::role::game_roles(num_players)?;
        Some(GameState {
            num_players,
            _roles,
        })
    }

    pub fn num_players(&self) -> u8 {
        self.num_players
    }
}

#[cfg(test)]
pub(crate) fn init_logging() {
    use std::sync::Once;

    static LOGGING: Once = Once::new();
    LOGGING.call_once(|| {
        env_logger::init();
    });
}
