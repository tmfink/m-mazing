use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoardAction {
    Escalator,
    Explore,
    Slide(Direction),
    Warp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    BoardAction(BoardAction),
    //DoSomething(PlayerId)
}
