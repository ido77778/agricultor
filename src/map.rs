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