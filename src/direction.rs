use ggez::event::{Keycode};
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

#[cfg(test)]
fn get_direction(key: Keycode) -> Direction
{
    Direction::from_keycode(key).unwrap()
}
#[test]
fn direction_from_keycode()
{
    let mut keycode = Keycode::Up;
    assert_eq!(Direction::Up, get_direction(keycode));
    keycode = Keycode::Down;
    assert_eq!(Direction::Down, get_direction(keycode));
    keycode = Keycode::Left;
    assert_eq!(Direction::Left, get_direction(keycode));
    keycode = Keycode::Right;
    assert_eq!(Direction::Right, get_direction(keycode));
}
