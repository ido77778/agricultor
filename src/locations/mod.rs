use crate::map::Map;

use self::simple_dungeon::generate_dungeon;

pub mod caves;
pub mod simple_dungeon;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;

pub fn generate_new_map() -> Map
{
    generate_dungeon(WIDTH, HEIGHT)
}