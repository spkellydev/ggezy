use ggez::{GameResult, Context, graphics};
use super::direction::Direction;

// ActorHandler trait is designed to help actors have common functionality
pub trait ActorHandler
{
    fn move_object(&mut self, direction: Direction);
    fn draw_object(&mut self, _context: &mut Context) -> GameResult<()>;
    fn clear_object(&mut self, context: &mut Context) 
    {
        // clear the actor from the screen
        graphics::clear(context);
    }
}