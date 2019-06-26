use ggez::graphics::{present, get_size, Color};
use ggez::{GameResult, Context};
use ggez::event::{EventHandler, Keycode, Mod};

use crate::player::Player;
use crate::direction::Direction;
use crate::actor_handler::ActorHandler;

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

