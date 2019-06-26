extern crate ggez;
use ggez::graphics::{Color, set_color, Point2, DrawMode, circle};
use ggez::{GameResult, Context, graphics};
use ggez::event::{EventHandler};

use crate::base::Base;
use crate::direction::Direction;
use crate::actor_handler::ActorHandler;

pub struct Player
{
    x: f32,
    y: f32,
    stride: f32,
    color: Color,
    velocity: Vec<f32>,
    base: Base,
}
impl Player
{
    pub fn new(w: f32, h: f32, color: Color) -> Player
    {
        let x = w / 2.0;
        let y = h / 2.0;
        let base = Base::new(w, h, color, 1);
        let velocity = vec!(50.0, 30.0);
        let stride = 0.3;
        Player { x, y, stride, color, velocity, base }
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
        let vel_x = self.velocity[0];
        let vel_y = self.velocity[1];
        match direction
        {
            Direction::Up => Some(self.y -= vel_y * self.stride),
            Direction::Down => Some(self.y += vel_y * self.stride),
            Direction::Left => Some(self.x -= vel_x * self.stride),
            Direction::Right => Some(self.x += vel_x * self.stride),
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

#[cfg(test)]
fn get_player() -> Player
{
    let val = 20.0;
    Player::new(val, val, Color::new(1.0, 1.0, 1.0, 1.0))
}
#[test]
fn player_new()
{
    let val = 20.0;
    let player = get_player();

    assert_eq!(player.x, val / 2.0);
    assert_eq!(player.y, val / 2.0);
    assert_eq!(player.color, Color::new(1.0, 1.0, 1.0, 1.0));
}
#[test]
fn player_move()
{
    let player = &mut get_player();
    let direction = Direction::Up;
    let movement = player.y - player.velocity[1] * player.stride;
    player.move_object(direction);

    assert_eq!(player.y, movement);
}
