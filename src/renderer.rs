use crate::json::{TileType};
use crate::map::{Map};

use std::char::from_u32;
use std::{collections::HashMap};
use rltk::{RGB, Rltk};

pub fn draw_level(tile_properties: &HashMap<u32, TileType>, map: &Map, ctx: &mut Rltk)
{
    // Indices
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while y < map.height as i32
    {
        // This should never panic.
        let tile = map.get_tile((x, y)).unwrap();

        let default_dirt = 
        TileType
        {
            id: 3,
            name: "Dirt".to_string(),
            glyph: from_u32(0x0000002E).unwrap(),
            rgb: (155, 118, 83),
            walkable: true,
            opaque: false
        };

        // If the tile id does not exist, use dirt instead.
        let tile_type =  tile_properties.get(&tile).unwrap_or(&default_dirt);
        let color = tile_type.rgb;
        // Drawing the tile.
        ctx.set(x, y, RGB::from_u8(color.0, color.1, color.2), RGB::from_f32(0., 0., 0.), rltk::to_cp437(tile_type.glyph));
    
        // Advancing the loop forward.
        x += 1;
        if x > map.width as i32
        {
            x = 0;
            y += 1;
        }
    }
}
