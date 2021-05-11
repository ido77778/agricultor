use std::{collections::HashMap, error, fs::File, io::BufReader, path::Path};

use serde::{Deserialize};
use rltk::RGB;

// Contains all useful JSON data.
pub struct JsonData
{
    pub tiles: HashMap<u32, TileType>
}

impl JsonData
{
    pub fn new() -> JsonData
    {
        let tiles_object = match get_tiles_hashmap()
        {
            Ok(hash) => hash,
            Err(e) =>
            {
                error!("Couldn't get tiles object - json::get_tiles_hashmap:\n{}", e);
                panic!("Couldn't get tiles object.")
            }
        };

        JsonData
        {
            tiles: tiles_object
        }
    }
}


/* 
//============//
//   Tiles    //
//============//
*/


// TODO: Add glyphs and RGBs to the JSON tiles.
// Contains the properties of a tile.
#[derive(Deserialize)]
pub struct TileType
{
    pub id: u32,
    pub name: String,
    pub glyph: String,
    pub rgb: RGB,
    pub walkable: bool,
}

pub fn get_tiles_hashmap() -> Result<HashMap<u32, TileType>, Box<dyn error::Error>>
{
    // Creates TileType objects from JSON and stores them in a hashmap.
    let current_dir = std::env::current_dir()?;
    warn!("{}", current_dir.display());
    let tiles_file = File::open(Path::new("./json/tiles.json"))?;
    let reader = BufReader::new(tiles_file);

    let mut tiles_hash = HashMap::new();
    let raw_data: DummyType = serde_json::from_reader(reader)?;

    for tile in raw_data.tiles
    {
        // TODO: Find a way to fix this.
        tiles_hash.insert
        (
            tile.id,
            TileType
            {
                id: tile.id,
                name: tile.name,
                glyph: tile.glyph,
                rgb: ,
                walkable: tile.walkable,
            }
        );
    }
    
    Ok(tiles_hash)
}

#[derive(Deserialize)]
struct DummyType
{
    tiles: Vec<TileType>
}