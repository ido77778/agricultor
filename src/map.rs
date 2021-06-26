use std::usize;
use rltk::{Algorithm3D, BaseMap, Point3};

use crate::json::JsonData;

pub struct Map
{
    pub tile_vector: Vec<u32>,
    pub json: JsonData,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub current_level: u32
}

impl Map
{
    pub fn new(json: JsonData) -> Map
    {
        let mut map = Map
        {
            tile_vector: vec![3; 80*50*40],
            json,
            width: 80,
            height: 50,
            depth: 40,
            current_level: 20
        };

        for x in 0..80 {
            map.set_tile((x, 0, 20), 0);
            map.set_tile((x, 49, 20), 0);
        }
        for y in 0..50 {
            map.set_tile((0, y, 20), 0);
            map.set_tile((79, y, 20), 0);
        }
        map
    }

    pub fn xyz_id(&self, point: (i32, i32, i32)) -> Option<usize>
    {
    // Returns a unique ID (scalar) for a 3D vector.
    // The formula is WIDTH*HEIGHT*z + WIDTH*y + x
    if point.0 | point.1 | point.2 < 0
    {
        // Return a None if any of the coordinates is negative.
        return None;
    }

    Some((point.2 as usize * (self.width * self.height) as usize) + (point.1 as usize * self.width as usize) + point.0 as usize)
    }

    pub fn get_tile(&self, point: (i32, i32, i32)) -> Option<u32>
    {
        // Getter for a single map tile.
        // We want to keep the game agnostic as to the actual representation of the map.
        let index = self.xyz_id(point)?;
        Some(self.tile_vector[index])
    }

    pub fn set_tile(&mut self, point: (i32, i32, i32), value: u32)
    {
        // Setter for a single map tile.
        let index = match self.xyz_id(point)
        {
            Some(index) => index,
            None => return
        };
        self.tile_vector[index] = value;
    }
}

impl BaseMap for Map
{
    fn is_opaque(&self, idx: usize) -> bool
    {
        if self.json.tiles.get(&(idx as u32)).unwrap().transparent == false
        { return true; } else { return false; }
    }    
}

impl Algorithm3D for Map
{
    fn dimensions(&self) -> rltk::Point3
    {
        Point3::new(self.width, self.height, self.depth)
    }
}
