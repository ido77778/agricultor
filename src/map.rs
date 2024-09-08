use std::hash::{Hash, Hasher};

use crate::json::JsonData;
use crate::prelude::*;
use crate::locations::simple_dungeon::Rect;

pub struct Map
{
    pub json: JsonData,
    pub map_vector: Vec<u32>,
    pub map_type: u32, // ID of the overmap tile. See overmap_tiles.json.
    pub rooms: Vec<Rect>,
    pub width: usize,
    pub height: usize,
    pub revealed_tiles: Vec<bool>,
}

impl Map
{
    pub fn get_tile(&self, tile: Point) -> &u32
    {
        // Getter for a single map tile. Retrieves the ID of the tile.
        // We want to keep the game agnostic as to the actual representation of the map.
        self.map_vector.get(tile.to_index(self.width)).expect("Tried to get a non-existent tile")
    }

    pub fn _set_tile(&mut self, tile: Point, value: u32)
    {
        // Setter for a single map tile.
        self.map_vector[tile.to_index(self.width)] = value;
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH as i32 && point.y >= 0 && point.y < SCREEN_HEIGHT as i32
    }

    pub fn can_player_walk(&self, tile: Point) -> bool
    {
        self.in_bounds(tile) && self.json.tiles[self.get_tile(tile)].walkable
    }
}

impl Algorithm2D for Map
{
    fn dimensions(&self) -> Point
    {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map
{
    fn is_opaque(&self, idx: usize) -> bool
    {
        self.json.tiles[&self.map_vector[idx]].opaque
    }
}

impl Hash for Map
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.map_vector.hash(state);
        self.map_type.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        self.revealed_tiles.hash(state);
    }
}