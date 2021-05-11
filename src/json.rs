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
    pub glyph: char,
    pub rgb: String,
    pub walkable: bool,
}

impl TileType
{
    pub fn get_rgb_from_string(&self) -> RGB
    {
        let green = "green".to_string();
        let brown = "brown".to_string();
        let burly = "burly".to_string();
        match &self.rgb
        {
            green => RGB::named(rltk::GREEN),
            brown => RGB::named(rltk::BROWN1),
            burly => RGB::named(rltk::BURLYWOOD),
            _ => RGB::named(rltk::WHITE)
        }
    }
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
        tiles_hash.insert
        (
            tile.id,
            tile
        );
    }
    
    Ok(tiles_hash)
}

#[derive(Deserialize)]
struct DummyType
{
    tiles: Vec<TileType>
}