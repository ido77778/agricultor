use std::{collections::HashMap, usize};
use rltk::{Algorithm3D, BaseMap, Point3};

use crate::json::TileType;


pub struct Map
{
    pub tile_vector: Vec<u32>,
    pub tile_properties: HashMap<u32, TileType>,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub current_level: u32
}

impl Map
{
    pub fn new(tile_properties: HashMap<u32, TileType>) -> Map
    {
        let mut map = Map
        {
            tile_vector: vec![3; 80*50*40],
            tile_properties,
            width: 80,
            height: 50,
            depth: 40,
            current_level: 20
        };

        for x in 0..80
        {
            map.set_tile(Point3::new(x, 0, 20), 0);
            map.set_tile(Point3::new(x, 49, 20), 0);
        }
        for y in 0..50
        {
            map.set_tile(Point3::new(0, y, 20), 0);
            map.set_tile(Point3::new(79, y, 20), 0);
        }
        map
    }

    pub fn xyz_id(&self, point: Point3) -> Option<usize>
    {
    // Returns a unique ID (scalar) for a 3D vector.
    // The formula is WIDTH*HEIGHT*z + WIDTH*y + x
    if point.x | point.y | point.z < 0
    {
        // Return a None if any of the coordinates is negative.
        return None;
    }

    Some((point.z as usize * (self.width * self.height) as usize) + (point.y as usize * self.width as usize) + point.x as usize)
    }

    pub fn get_tile(&self, point: Point3) -> Option<u32>
    {
        // Getter for a single map tile.
        // We want to keep the game agnostic as to the actual representation of the map.
        let index = self.xyz_id(point)?;
        Some(self.tile_vector[index])
    }

    pub fn set_tile(&mut self, point: Point3, value: u32)
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
        if self.tile_properties.get(&(idx as u32)).unwrap().transparent == false
        { return true; } else { return false; }
    }    
}

impl Algorithm3D for Map
{
    fn dimensions(&self) -> Point3
    {
        Point3::new(self.width, self.height, self.depth)
    }
}
