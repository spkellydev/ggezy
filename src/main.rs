use std::fs::File;

extern crate ggez;
extern crate rand;
use ggez::event;
use ggez::{Context};
use ggez::conf::Conf;

mod base;
mod player;
mod direction;
mod actor_handler;
mod state;

use state::GameState;

fn main() {
    let mut file = File::open("conf.toml").unwrap();
    let config = Conf::from_toml_file(&mut file).unwrap();
    let context = &mut Context::load_from_conf("random_walkers", "Sean Kelly", config).unwrap();
    let game_state = &mut GameState::new(context);

    event::run(context, game_state).unwrap();
}
