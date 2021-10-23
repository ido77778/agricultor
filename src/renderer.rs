use crate::components::{Player, Viewshed};
use crate::map::{Map};
use crate::JSON;

use rltk::{Point, RGB, Rltk};
use specs::prelude::*;

pub fn draw_level(ecs: &World, ctx: &mut Rltk)
{
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, viewshed) in (&mut players, &mut viewsheds).join() 
    {
        // Indices
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        while y < map.height as i32
        {
            // Only draw tiles if in viewshed.
            let pt = Point::new(x,y);
            if map.revealed_tiles[pt.to_index(map.width)]
            {
                // This should never panic.
                let tile = map.get_tile((x, y)).unwrap();

                let color = JSON.with(|data|{ data.tiles[&tile].rgb });
                let glyph = JSON.with(|data|{ data.tiles[&tile].glyph });
                // Drawing the tile.
                ctx.set(x, y, RGB::from_u8(color.0, color.1, color.2), RGB::from_f32(0., 0., 0.), rltk::to_cp437(glyph));

                if !map.visible_tiles[pt.to_index(map.width)]
                {
                    ctx.set(x, y, RGB::from_u8(color.0, color.1, color.2).to_greyscale(), RGB::from_f32(0., 0., 0.), rltk::to_cp437(glyph));
                }
            }

            // Advancing the loop forward.
            x += 1;
            if x > map.width as i32
            {
                x = 0;
                y += 1;
            }
        }
    }
}
