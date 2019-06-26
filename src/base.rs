extern crate ggez;
use ggez::graphics::{Color, set_color, Point2, DrawMode, circle};
use ggez::{GameResult, Context};

use crate::direction::Direction;
use crate::actor_handler::ActorHandler;

pub struct Base
{
    x: f32,
    y: f32,
    color: Color,
    grid: Vec<Vec<u8>>,
    pub player_id: i64
}
impl Base
{
    /**
     * posit:
     *  0 0 0 0 0 0 0 0 0 
     *  0 0 0 0 0 0 0 0 0  The grid should fan out around the center of the Base
     *  0 0 0 1 1 1 0 0 0  and grow as the player captures more territory
     *  0 0 0 1 1 1 0 0 0
     *  0 0 0 1 1 1 0 0 0  When territory is captured, the zeros turn to ones and the grid expands
     *  0 0 0 0 0 0 0 0 0  Base starts as a standard circle but can grow to any shape
     *  0 0 0 0 0 0 0 0 0
     */
    pub fn new(w: f32, h: f32, color: Color, player_id: i64) -> Base
    {
        let x = w / 2.0;
        let y = h / 2.0;
        let grid = vec!{
            vec!{ 0, 0, 0 }, vec!{ 0, 0, 0 }, vec!{ 0, 0, 0 },
            vec!{ 1, 1, 1 }, vec!{ 1, 1, 1 }, vec!{ 1, 1, 1 },
            vec!{ 0, 0, 0 }, vec!{ 0, 0, 0 }, vec!{ 0, 0, 0 },
            };
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

        // the base needs to update in size as we conquer more land
        // when that happens, the grid expands to encompass the surrounded pixels... in theory
        let mut size = 30;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                // update the base location
                let location: Point2 = Point2::new(self.x + i as f32, self.y + j as f32);

                set_color(_context, self.color)?;
                circle(_context, DrawMode::Fill, location, size as f32, 1.0)?;
                size += *cell;
            }
        }
        Ok(())
    }
}
