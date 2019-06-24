// Directions we can travel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up, Down, Left, Right
}
impl Direction {
    // helper function that will convert a keycode to a direction
    pub fn from_keycode(key: Keycode) -> Option<Direction> {
        match key {
            Keycode::Up => Some(Direction::Up),
            Keycode::Down => Some(Direction::Down),
            Keycode::Left => Some(Direction::Left),
            Keycode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}