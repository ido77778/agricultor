use crate::json::TileType;

use std::{collections::HashMap};
use rltk::{RGB, Rltk};

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;

pub struct Map
{
    pub map_vector: Vec<u32>,
    pub current_level: u32
}

impl Map
{
    pub fn new() -> Map
    {
        let mut map = vec![3; 80*50*40];
        for x in 0..80 {
            map[xyz_id(x, 0, 0)] = 0;
            map[xyz_id(x, 49, 0)] = 0;
        }
        for y in 0..50 {
            map[xyz_id(0, y, 0)] = 0;
            map[xyz_id(79, y, 0)] = 0;
        }

        Map
        {
            map_vector: map,
            current_level: 20
        }
    }
}

pub fn xyz_id(x: i32, y:i32, z:i32) -> usize
{
    // Returns a unique ID (scalar) for a 3D vector.
    // The formula is WIDTH*HEIGHT*z + WIDTH*y + x where HEIGHT = 50, WIDTH = 80.
    (z as usize * WIDTH * HEIGHT) + (y as usize * WIDTH) + x as usize
}

pub fn draw_level(tile_properties: &HashMap<u32, TileType>, map: &Map, ctx: &mut Rltk)
{
    // Indices
    let mut x = 0;
    let mut y = 0;

    for tile in &map.map_vector
    {
        if (tile/(WIDTH*HEIGHT) as u32) == map.current_level
        {
            // Drawing the tile.
            let tile_type =  tile_properties.get(&tile).unwrap();
            let color = tile_type.rgb;
            ctx.set(x, y, RGB::from_u8(color.0, color.1, color.2), RGB::from_f32(0., 0., 0.), rltk::to_cp437(tile_type.glyph));

            // Advancing the loop forward.
            x += 1;
            if x > 79
            {
                x = 0;
                y += 1;
            }
        }
    }
}