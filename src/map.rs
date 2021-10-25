use rltk::{Algorithm2D, BaseMap, Point};

use crate::{JSON};
use crate::locations::caves::generate_cavern;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;

pub struct Map
{
    pub map_vector: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>
}

impl Map
{
    pub fn new() -> Map
    {
        // let mut map = vec![1; (WIDTH+1)*(HEIGHT+1)];
        // for x in 0..80 {
        //     map[Map::xy_id((x, 0)).unwrap()] = 0;
        //     map[Map::xy_id((x, 49)).unwrap()] = 0;
        // }
        // for y in 0..50 {
        //     map[Map::xy_id((0, y)).unwrap()] = 0;
        //     map[Map::xy_id((79, y)).unwrap()] = 0;
        // }
        
        let map = generate_cavern(WIDTH, HEIGHT);

        Map
        {
            map_vector: map,
            width: WIDTH,
            height: HEIGHT,
            revealed_tiles: vec![false; (WIDTH+1)*(HEIGHT+1)],
            visible_tiles: vec![false; (WIDTH+1)*(HEIGHT+1)]
        }
    }

    pub fn get_tile(&self, tile: (i32, i32)) -> Option<u32>
    {
        // Getter for a single map tile.
        // We want to keep the game agnostic as to the actual representation of the map.
        let index = Map::xy_id(tile);

        // warn!("{}", index);

        Some(self.map_vector[index])
    }

    pub fn set_tile(&mut self, tile: (i32, i32), value: u32)
    {
        // Setter for a single map tile.
        self.map_vector[Map::xy_id(tile)] = value;
    }

    pub fn xy_id(tile: (i32, i32)) -> usize
    {
        // Returns a unique ID (scalar) for a 3D vector.
        // The formula is WIDTH*y + x
        if tile.0 | tile.1 < 0
        {
            panic!("FUGGGGG NEGATIVE TILES REEEE")
        }

        (tile.1 as usize * WIDTH) + tile.0 as usize
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
        JSON.with
        (|data| { data.tiles[&self.map_vector[idx]].opaque })
    }
}