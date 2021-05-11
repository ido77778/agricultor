use crate::json::TileType;

use std::collections::HashMap;
use rltk::{RGB, Rltk};

pub fn xyz_id(x: i32, y:i32, z:i32) -> usize
{
    // Returns a unique ID (scalar) for a 3D vector.
    // The formula is WIDTH*HEIGHT*z + WIDTH*y + x where HEIGHT = 50, WIDTH = 80.
    (z as usize * 80 * 50) + (y as usize * 80) + x as usize
}

// TODO: Make a basic map.
pub fn new_map() -> Vec<u32>
{
    let mut map = vec![3; 80*50];
    for x in 0..80 {
        map[xyz_id(x, 0, 0)] = 0;
        map[xyz_id(x, 49, 0)] = 0;
    }
    for y in 0..50 {
        map[xyz_id(0, y, 0)] = 0;
        map[xyz_id(79, y, 0)] = 0;
    }
    map
}

pub fn draw_map(tile_properties: &HashMap<u32, TileType>, map: &[u32], ctx: &mut Rltk)
{
    // Indices
    let mut x = 0;
    let mut y = 0;

    for tile in map
    {
        // Drawing the tile.
        let tile_type =  tile_properties.get(tile).unwrap();
        ctx.set(x, y, tile_type.get_rgb_from_string(), RGB::from_f32(0., 0., 0.), rltk::to_cp437(tile_type.glyph));

        // Advancing the loop forward.
        x += 1;
        if x > 79
        {
            x = 0;
            y += 1;
        }
    }
}