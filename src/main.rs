mod components;
mod state;
mod systems;
mod json;
mod map;
mod locations;
mod camera;
mod spawner;
mod overmap;
mod turn_state;
mod prelude
{
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use rltk::prelude::*;
    pub use crate::systems::*;
    pub use crate::spawner::*;
    pub use crate::components::*;
    pub use crate::turn_state::*;

    pub const SCREEN_WIDTH: u8 = 80;
    pub const SCREEN_HEIGHT: u8 = 50;
    pub const DISPLAY_WIDTH: u8 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: u8 = SCREEN_HEIGHT / 2;
}

use state::State;
use prelude::*;

#[macro_use]
extern crate log;

fn main() -> rltk::BError
{
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut context = rltk::BTermBuilder::new()
        .with_title("Agricultor")
        .with_fps_cap(60.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(24, 24)
        .with_resource_path("resources/")
        .with_font("bisasam_24x24.png", 24, 24)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "bisasam_24x24.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "bisasam_24x24.png")
        .build()?;

    context.with_post_scanlines(false);

    main_loop(context, State::new())
}
