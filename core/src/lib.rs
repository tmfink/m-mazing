pub mod action;
pub mod role;
pub mod tile;

pub struct PlayerId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pawn {
    Green,
    Orange,
    Yellow,
    Purple,
}

pub struct GameState {
    num_players: u8,
    roles: &'static [&'static [action::BoardAction]],
}

impl GameState {
    pub fn new(num_players: u8) -> Option<Self> {
        let roles = crate::role::game_roles(num_players)?;
        Some(GameState { num_players, roles })
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
