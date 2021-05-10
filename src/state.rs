use crate::{components::*, tiles::TileType};
use crate::player::player_input;

use rltk::{GameState, Rltk};
use specs::{Join, World, WorldExt};
use std::collections::HashMap;

pub struct State
{
    pub ecs: World,
    pub tile_types: HashMap<u32, TileType>
}

impl GameState for State
{
    fn tick(&mut self, ctx: &mut Rltk)
    {
        ctx.cls();

        player_input(self, ctx);

        // Reads from storage.
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        // Renders all objects with both Position and Renderable.
        for (pos, render) in (&positions, &renderables).join()
        {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}