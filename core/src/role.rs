use crate::prelude::*;

use BoardAction::*;

pub fn game_roles(num_players: u8) -> Option<&'static [&'static [BoardAction]]> {
    if num_players == 0 {
        return None;
    }
    ALLOWED_ACTIONS.get(num_players as usize).copied()
}

const ALLOWED_ACTIONS: &[&[&[BoardAction]]] = &[
    // 0 players
    &[],
    // 1 player
    &[&[
        Warp,
        Explore,
        Escalator,
        Slide(CartesianDirection::Left),
        Slide(CartesianDirection::Up),
        Slide(CartesianDirection::Down),
        Slide(CartesianDirection::Right),
    ]],
    // 2 players
    &[
        &[
            Escalator,
            Explore,
            Slide(CartesianDirection::Down),
            Slide(CartesianDirection::Left),
        ],
        &[
            Warp,
            Slide(CartesianDirection::Up),
            Slide(CartesianDirection::Right),
        ],
    ],
];
