pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;
pub const DEPTH: usize = 40;

pub struct Map
{
    pub map_vector: Vec<u32>,
    pub current_level: u32
}

impl Map
{
    pub fn new() -> Map
    {
        let mut map = vec![3; WIDTH*HEIGHT*DEPTH];
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

    pub fn get_tile(&self, x: i32, y: i32, z: i32) -> u32
    {
        // Getter for a single map tile.
        // We want to keep the game agnostic as to the actual representation of the map.
        self.map_vector[xyz_id(x, y, z)]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, z: i32, value: u32)
    {
        // Setter for a single map tile.
        self.map_vector[xyz_id(x, y, z)] = value;
    }
}

pub fn xyz_id(x: i32, y:i32, z:i32) -> usize
{
    // Returns a unique ID (scalar) for a 3D vector.
    // The formula is WIDTH*HEIGHT*z + WIDTH*y + x
    (z as usize * WIDTH * HEIGHT) + (y as usize * WIDTH) + x as usize
}