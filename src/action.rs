#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

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
