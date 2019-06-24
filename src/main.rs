use std::fs::File;

extern crate ggez;
extern crate rand;
use ggez::event;
use ggez::{Context, graphics::{get_size, Color}};
use ggez::conf::Conf;

mod actors;
use actors::Player;

fn main() {
    let mut file = File::open("conf.toml").unwrap();
    let config = Conf::from_toml_file(&mut file).unwrap();
    let context = &mut Context::load_from_conf("random_walkers", "Sean Kelly", config).unwrap();

    let (width, height) = get_size(context);
    let player_object = &mut Player::new(width as f32, height as f32, Color::new(1.0, 1.0, 1.0, 1.0));
    
    event::run(context, player_object).unwrap();
}



// extern crate rand;
// extern crate ggez;

// use std::fs::File;

// use ggez::event;
// use ggez::event::{Keycode, Mod};
// use ggez::graphics::{circle, DrawMode, Point2, present};
// use ggez::{GameResult, Context, graphics::get_size};
// use ggez::conf::Conf;

// // Directions we can travel
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// enum Direction {
//     Up, Down, Left, Right
// }
// impl Direction {
//     // helper function that will convert a keycode to a direction
//     pub fn from_keycode(key: Keycode) -> Option<Direction> {
//         match key {
//             Keycode::Up => Some(Direction::Up),
//             Keycode::Down => Some(Direction::Down),
//             Keycode::Left => Some(Direction::Left),
//             Keycode::Right => Some(Direction::Right),
//             _ => None,
//         }
//     }
// }

// struct GameState {
//     x: f32,
//     y: f32
// }
// impl GameState {
//     fn new(width: f32, height: f32) -> GameState {
//         let x = rand::random::<f32>() * width;
//         let y = rand::random::<f32>() * height;
//         GameState {
//             x,
//             y
//         }
//     }
// }

// impl event::EventHandler for GameState {
//     fn update(&mut self, _context: &mut Context) -> GameResult<()> {
//         let random_number = rand::random::<f32>();

//         if random_number > 0.75 {
//             self.y -= 1.0;
//         } else if random_number > 0.5 {
//             self.x += 1.0;
//         } else if random_number > 0.25 {
//             self.y += 1.0;
//         } else {
//             self.x -= 1.0;
//         }

//         Ok(())
//     }
//     fn draw(&mut self, _context: &mut Context) -> GameResult<()> {
//         let location: Point2 = Point2::new(self.x, self.y);
//         circle(_context, DrawMode::Fill, location, 10.0, 1.0)?;
//         present(_context);
//         Ok(())
//     }

//     fn key_down_event(
//         &mut self,
//         ctx: &mut Context, 
//         keycode: Keycode, 
//         _keymod: Mod, 
//         _repeat: bool
//     ) {
//         if let Some(_dir) = Direction::from_keycode(keycode) {
//             self.x = 40.0
//         }
//     }
// }

// fn main() {
//     let mut file = File::open("conf.toml").unwrap();
//     let config = Conf::from_toml_file(&mut file).unwrap();
//     let context = &mut Context::load_from_conf("random_walkers", "Sean Kelly", config).unwrap();

//     let (width, height) = get_size(context);
//     let game_state = &mut GameState::new(width as f32, height as f32);
    
//     event::run(context, game_state).unwrap();
// }
