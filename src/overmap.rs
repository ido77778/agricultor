use crate::prelude::*;

pub struct MapMetadata
{
    pub hash: u8, // MD5 hash of the map.
    pub overmap_tile: u8,
}

pub struct Overmap
{
    maps: Vec<MapMetadata>
}

impl Overmap
{
    pub fn new() -> Self
    {
        Overmap
        {
            maps: Vec::with_capacity(60*60)
        }
    }
}