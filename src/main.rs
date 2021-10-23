mod components;
mod visibility_system;
mod state;
mod json;
mod player;
mod map;
mod renderer;
mod visibility_system;

use player::create_player;
use components::*;
use json::JsonData;
use map::Map;
use state::State;

use specs::{ World, WorldExt };

#[macro_use]
extern crate log;

thread_local! {pub static JSON: JsonData = JsonData::new() }

fn main() -> rltk::BError
{
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Agricultor")
        .build()?;
    
    let mut gs = State { ecs: World::new(), json: JsonData::new()}; // Gamestate
    let json = JsonData::new();

    // Register the components.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    gs.ecs.insert(Map::new(json.tiles));

    create_player(&mut gs, 40, 25);

    rltk::main_loop(context, gs)
}
