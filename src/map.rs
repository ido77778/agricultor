use crate::tiles::TileType;

pub fn xyz_id(x: i32, y:i32, z:i32) -> usize
{
    // Returns a unique ID (scalar) for a 3D vector.
    // The formula is WIDTH*HEIGHT*z + WIDTH*y + x where HEIGHT = 50, WIDTH = 80.
    (z as usize * 80 * 50) + (y as usize * 80) + x as usize
}

// TODO: Make a basic map.
pub fn new_map() -> Vec<TileType>
{
    let mut map = vec![TileType];
    map
}