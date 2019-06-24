use ggez::graphics::{present, get_size, Color};
use ggez::{GameResult, Context};
use ggez::event::{EventHandler, Keycode, Mod};

use actors::Player;
use controls::Direction;

// GameState will serve as proxy / intermediate for other game object
// utilizes ggez::event::EventHandler for handle canvas updates
pub struct GameState
{
    player: Player
}
impl GameState
{
    pub fn new(_context: &Context) -> GameState
    {
        let (width, height) = get_size(_context);
        let player_object = Player::new(width as f32, height as f32, Color::new(1.0, 1.0, 1.0, 1.0));
        GameState { player: player_object }
    }
}
impl EventHandler for GameState
{
    fn update(&mut self, _context: &mut Context) -> GameResult<()>
    {
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult<()>
    {
        // add player to the game context
        self.player.draw(_context)?;
        // draw to the canvas with the current context
        present(_context);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context, 
        keycode: Keycode, 
        _keymod: Mod, 
        _repeat: bool
    )
    {
        // if the keycode matches a direction object, move the player
        if let Some(direction) = Direction::from_keycode(keycode)
        {
            self.player.handle_movement(direction);
        }
        else if keycode == Keycode::Escape
        {
            println!("Game exiting normally");
            std::process::exit(1);
        }
    }
}

pub mod actors
{
    extern crate ggez;
    use ggez::graphics::{Color, set_color, Point2, DrawMode, circle};
    use ggez::{GameResult, Context, graphics};
    use ggez::event::{EventHandler};
    use super::controls::Direction;

    pub struct Player
    {
        x: f32,
        y: f32,
        color: Color,
        velocity: f32
    }
    impl Player
    {
        pub fn new(w: f32, h: f32, color: Color) -> Player
        {
            let x = w / 2.0;
            let y = h / 2.0;
            Player { x, y, color, velocity: 20.0 }
        }

        // move the player
        pub fn handle_movement(&mut self, direction: Direction)
        {
            match direction
            {
                Direction::Up => Some(self.y -= self.velocity * 0.3),
                Direction::Down => Some(self.y += self.velocity * 0.3),
                Direction::Left => Some(self.x -= self.velocity * 0.3),
                Direction::Right => Some(self.x += self.velocity * 0.3),
            };
        }
    }
    impl EventHandler for Player
    {
        fn update(&mut self, _context: &mut Context) -> GameResult<()>
        {
            Ok(())
        }

        fn draw(&mut self, _context: &mut Context) -> GameResult<()>
        {
            // clear the previous drawing of the player
            graphics::clear(_context);
            // update the player location
            let location: Point2 = Point2::new(self.x, self.y);
            
            // draw the player object
            set_color(_context, self.color)?;
            circle(_context, DrawMode::Fill, location, 10.0, 1.0)?;
            Ok(())
        }
    }
}

pub mod controls
{
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
}