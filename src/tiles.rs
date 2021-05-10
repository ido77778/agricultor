use std::{collections::HashMap, error, fs::File, io::BufReader, path::Path};

use serde::{Deserialize};

// TODO: Add glyphs and RGBs to the JSON tiles.
#[derive(Deserialize)]
pub struct TileType
{
    pub id: u32,
    pub name: String,
    pub glyph: String,
    pub walkable: bool,
}

#[derive(Deserialize)]
struct Test
{
    tiles: Vec<TileType>
}

pub fn get_tiles_hashmap() -> Result<HashMap<u32, TileType>, Box<dyn error::Error>>
{
    let current_dir = std::env::current_dir()?;
    warn!("{}", current_dir.display());
    let tiles_file = File::open(Path::new("./src/json/tiles.json"))?;
    let reader = BufReader::new(tiles_file);

    let mut tiles_hash = HashMap::new();
    let raw_data: Test = serde_json::from_reader(reader)?;

    for tile in raw_data.tiles
    {
        tiles_hash.insert
        (
            tile.id,
            tile
        );
    }
    
    return Ok(tiles_hash);
}