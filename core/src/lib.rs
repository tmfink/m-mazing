pub mod action;
pub mod role;
pub mod scenario;
pub mod tile;

pub mod prelude {
    pub use crate::action::*;
    pub use crate::role::*;
    pub use crate::scenario::*;
    pub use crate::tile::{
        cell::*, direction::*, escalator::*, grid_coord::*, tileset::*, wall::*, *,
    };
    pub use crate::*;
    pub use log::*;

    cfg_if! {
        if #[cfg(feature = "gui")] {
            pub use crate::render::{camera::*, shape::*, theme::*, *};
            pub use bevy_prototype_lyon::prelude::*;
        }
    }
}

cfg_if! {
    if #[cfg(feature = "gui")] {
        pub use bevy;
        pub use bevy_prototype_lyon;

        pub mod render;
        use bevy::prelude::*;
    }
}

use cfg_if::cfg_if;

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
