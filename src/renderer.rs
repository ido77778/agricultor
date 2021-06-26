use crate::json::TileType;
use crate::map::{Map, WIDTH, HEIGHT};

use std::{collections::HashMap};
use rltk::{RGB, Rltk};

pub fn draw_level(tile_properties: &HashMap<u32, TileType>, map: &Map, ctx: &mut Rltk)
{
    // Indices
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while y < HEIGHT as i32
    {
        // This should never panic.
        let tile = map.get_tile((x, y, map.current_level as i32)).unwrap();

        // Drawing the tile.
        let tile_type =  tile_properties.get(&tile).unwrap();
        let color = tile_type.rgb;
        ctx.set(x, y, RGB::from_u8(color.0, color.1, color.2), RGB::from_f32(0., 0., 0.), rltk::to_cp437(tile_type.glyph));
    
        // Advancing the loop forward.
        x += 1;
        if x > WIDTH as i32
        {
            x = 0;
            y += 1;
        }
    }
}
