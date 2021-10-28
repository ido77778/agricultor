use crate::prelude::*;
use crate::locations::caves::generate_cavern;
use crate::spawner::create_player;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;

pub struct Map
{
    pub map_vector: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub revealed_tiles: Vec<bool>,
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
        }
    }

    pub fn get_tile(&self, tile: Point) -> &u32
    {
        // Getter for a single map tile.
        // We want to keep the game agnostic as to the actual representation of the map.

        error!("Tried to get non-existent tile: ({}, {})", tile.x, tile.y);
        self.map_vector.get(tile.to_index(self.width)).expect("Tried to get a non-existent tile")
    }

    pub fn set_tile(&mut self, tile: Point, value: u32)
    {
        // Setter for a single map tile.
        self.map_vector[tile.to_index(self.width)] = value;
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH as i32 && point.y >= 0 && point.y < SCREEN_HEIGHT as i32
    }

    pub fn can_player_walk(&self, tile: Point) -> bool
    {
        self.in_bounds(tile) && JSON.with(|data| { data.tiles[self.get_tile(tile)].walkable })
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