extern crate ggez;
use ggez::graphics::{Color, set_color, Point2, DrawMode, present, circle};
use ggez::event::{EventHandler, Keycode, Mod};
use ggez::{GameResult, Context, graphics};

include!("controls.rs");

pub struct Player
{
    x: f32,
    y: f32,
    color: Color
}
impl Player
{
    pub fn new(w: f32, h: f32, color: Color) -> Player
    {
        let x = w / 2.0;
        let y = h / 2.0;
        Player { x, y, color }
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
        graphics::clear(_context);
        let location: Point2 = Point2::new(self.x, self.y);
        set_color(_context, self.color)?;
        circle(_context, DrawMode::Fill, location, 10.0, 1.0)?;
        present(_context);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context, 
        keycode: Keycode, 
        _keymod: Mod, 
        _repeat: bool
    )
    {
        if let Some(direction) = Direction::from_keycode(keycode)
        {          
            match direction
            {
                Direction::Up => Some(self.y -= 20.0 * 0.4),
                Direction::Down => Some(self.y += 20.0 * 0.4),
                Direction::Left => Some(self.x -= 20.0 * 0.4),
                Direction::Right => Some(self.x += 20.0* 0.4),
            };
        }
        else if keycode == Keycode::Escape
        {
            std::process::exit(1);
        }
    }
}