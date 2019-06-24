use ggez::graphics::{present, get_size, Color};
use ggez::{GameResult, Context};
use ggez::event::{EventHandler, Keycode, Mod};

use actors::{Player, ActorHandler};
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
            self.player.move_object(direction);
        }
        else if keycode == Keycode::Escape
        {
            self.player.clear_object(_context);
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

    // ActorHandler trait is designed to help actors have common functionality
    pub trait ActorHandler
    {
        fn move_object(&mut self, direction: Direction);
        fn draw_object(&mut self, _context: &mut Context) -> GameResult<()>;
        fn clear_object(&mut self, context: &mut Context) 
        {
            // clear the player from the screen
            graphics::clear(context);
        }
    }

    pub struct Player
    {
        x: f32,
        y: f32,
        color: Color,
        velocity: f32,
        base: Base,
    }
    impl Player
    {
        pub fn new(w: f32, h: f32, color: Color) -> Player
        {
            let x = w / 2.0;
            let y = h / 2.0;
            let base = Base::new(w, h, color, 1);
            Player { x, y, color, velocity: 20.0, base }
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
            self.draw_object(_context)?;
            self.base.draw_object(_context)?;
            Ok(())
        }
    }
    impl ActorHandler for Player
    {
        // move the player
        fn move_object(&mut self, direction: Direction)
        {
            match direction
            {
                Direction::Up => Some(self.y -= self.velocity * 0.3),
                Direction::Down => Some(self.y += self.velocity * 0.3),
                Direction::Left => Some(self.x -= self.velocity * 0.3),
                Direction::Right => Some(self.x += self.velocity * 0.3),
            };
        }

        fn draw_object(&mut self, _context: &mut Context) -> GameResult<()>
        {
            // update the player location
            let location: Point2 = Point2::new(self.x, self.y);

            // draw the player object
            set_color(_context, self.color)?;
            circle(_context, DrawMode::Fill, location, 15.0, 1.0)?;

            Ok(())
        }

        fn clear_object(&mut self, context: &mut Context)
        {
            graphics::clear(context);
            if self.base.player_id == 1 {
                self.base.clear_object(context);
            }
        }
    }

    pub struct Base
    {
        x: f32,
        y: f32,
        color: Color,
        grid: Vec<Vec<u8>>,
        player_id: i64
    }
    impl Base
    {
        pub fn new(w: f32, h: f32, color: Color, player_id: i64) -> Base
        {
            let x = w / 2.0;
            let y = h / 2.0;
            let grid = vec!{ vec!{ 1, 1, 1 }, vec!{ 1, 1, 1 }, vec!{ 1, 1, 1 } };
            Base { x, y, color, grid, player_id }
        }
    }
    impl ActorHandler for Base
    {
        fn move_object(&mut self, _direction: Direction)
        {
        }

        fn draw_object(&mut self, _context: &mut Context) -> GameResult<()>
        {
            // update the base location
            let location: Point2 = Point2::new(self.x, self.y);

            // the base needs to update in size as we conquer more land
            // when that happens, the grid expands to encompass the surrounded pixels... in theory
            let mut size = 30;
            for row in &self.grid {
                for cell in row {
                    size += *cell;
                }
            }

            set_color(_context, self.color)?;
            circle(_context, DrawMode::Fill, location, size as f32, 1.0)?;
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