use crate::{prelude::*, map::Map};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct MapMetadata
{
    pub hash: u64, // MD5 hash of the map.
    pub overmap_tile: u32,
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

    pub fn store_map(&mut self, map: &Map)
    {
        let mut hasher = DefaultHasher::new();
        map.hash(&mut hasher);

        let metadata = MapMetadata
        {
            hash: hasher.finish(),
            overmap_tile: map.map_type
        };

        self.maps.push(metadata);
    }

    pub fn _get_metadata(self, point: Point) -> MapMetadata
    {
        *self.maps.get(point.to_index(60)).expect("Tried to get a non-existent overmap tile.")
    }
}