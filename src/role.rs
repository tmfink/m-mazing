use crate::action::*;

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
    &[
        &[
            BoardAction::Escalator,
            BoardAction::Explore,
            BoardAction::Slide(Direction::South),
            BoardAction::Slide(Direction::West),
        ],
        &[
            BoardAction::Warp,
            BoardAction::Slide(Direction::North),
            BoardAction::Slide(Direction::East),
        ],
    ],
];
