use rltk::{Algorithm2D, BaseMap, Point};

use crate::JSON;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;

pub struct Map
{
    pub map_vector: Vec<u32>,
    width: usize,
    height: usize
}

impl Map
{
    pub fn new(tile_properties: HashMap<u32, TileType>) -> Map
    {
        let mut map = vec![3; (WIDTH+1)*(HEIGHT+1)];
        for x in 0..80 {
            map[xy_id((x, 0)).unwrap()] = 0;
            map[xy_id((x, 49)).unwrap()] = 0;
        }
        for y in 0..50 {
            map[xy_id((0, y)).unwrap()] = 0;
            map[xy_id((79, y)).unwrap()] = 0;
        }

        Map
        {
            map_vector: map,
            width: WIDTH,
            height: HEIGHT
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

    pub fn get_tile(&self, tile: (i32, i32)) -> Option<u32>
    {
        // Getter for a single map tile.
        // We want to keep the game agnostic as to the actual representation of the map.
        let index = xy_id(tile)?;

        // warn!("{}", index);

        Some(self.map_vector[index])
    }

    pub fn set_tile(&mut self, tile: (i32, i32), value: u32)
    {
        // Setter for a single map tile.
        let index = match xy_id(tile)
        {
            Some(index) => index,
            None => return
        };
        self.tile_vector[index] = value;
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
        (
            |data|
            {
                data.tiles[&self.map_vector[idx]].opaque
            }
        )
    }
}

fn xy_id(tile: (i32, i32)) -> Option<usize>
{
    // Returns a unique ID (scalar) for a 3D vector.
    // The formula is WIDTH*HEIGHT*z + WIDTH*y + x
    if tile.0 | tile.1 < 0
    {
        // Return a None if any of the coordinates is negative.
        return None;
    }
    // warn!("{}", (tile.1 as usize * WIDTH) + tile.0 as usize);
    Some((tile.1 as usize * WIDTH) + tile.0 as usize)
}
