pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;
pub const DEPTH: usize = 40;

pub struct Map
{
    pub map_vector: Vec<u32>,
    pub current_level: i32
}

impl Map
{
    pub fn new() -> Map
    {
        let map = vec![2; WIDTH*HEIGHT*DEPTH];
        for x in 0..80 {
            map[xyz_id(x, 0, 20)] = 0;
            map[xyz_id(x, 49, 20)] = 0;
        }
        for y in 0..50 {
            map[xyz_id(0, y, 20)] = 0;
            map[xyz_id(79, y, 20)] = 0;
        }

        Map
        {
            map_vector: map,
            current_level: 20
        }
    }

    pub fn get_tile(&self, tile: (i32, i32, i32)) -> Option<u32>
    {
        // Getter for a single map tile.
        // We want to keep the game agnostic as to the actual representation of the map.
        if tile.0 | tile.1 | tile.2 < 0
        {
            // Return a None if any of the coordinates is negative.
            return None;
        }

        let index = (tile.2 as usize * WIDTH * HEIGHT) + (tile.1 as usize * WIDTH) + tile.0 as usize;
        Some(self.map_vector[index])
    }

    pub fn set_tile(&mut self, tile: (i32, i32, i32), value: u32)
    {
        let index = (tile.2 as usize * WIDTH * HEIGHT) + (tile.1 as usize * WIDTH) + tile.0 as usize;
        self.map_vector[index] = value;
    }
}