mod components;
mod state;
mod tiles;
mod player;
mod map;

use player::create_player;
use components::*;
use state::State;
use map::new_map;

use specs::{ World, WorldExt };

#[macro_use]
extern crate log;

fn main() -> rltk::BError
{
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Agricultor")
        .build()?;

    let tiles_object = match tiles::get_tiles_hashmap()
    {
        Ok(hash) => hash,
        Err(e) =>
        {
            error!("Couldn't get tiles object - tiles::get_tiles_hashmap:\n{}", e);
            panic!("Couldn't get tiles object.")
        }
    };
    let mut gs = State { ecs: World::new(),  tile_types:  tiles_object}; // Gamestate
    // Register the components.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    create_player(&mut gs, 40, 25, 0);

    rltk::main_loop(context, gs)
}