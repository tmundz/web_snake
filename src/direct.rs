//classifies direction events for the snake
#[derive(Debug,PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    //will not allow the snake to move in an oppisite direction
    // ex west then east or north then south
    //returns a bool
    pub fn oppisite(self, other: Direction) -> bool {
        self == Direction::North && other == Direction::South
        || self == Direction::South && other == Direction::North
        || self == Direction::West && other == Direction::East
        || self == Direction::East && other == Direction::West
    }
}