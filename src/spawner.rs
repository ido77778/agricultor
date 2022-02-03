use std::collections::HashSet;

use crate::prelude::*;
use crate::{map::Map};

pub fn _find_walkable(map: &Map, pos: Point) -> Point
{
    // If the supplied position isn't walkable, we iterate over the map
    // until we find a walkable tile.
    let mut x = 0;
    let mut y = 0;
    if !JSON.with(|data| { data.tiles[map.get_tile(pos)].walkable })
    {
        while !JSON.with
        (
            |data|
            {
                data.tiles[map.get_tile(Point::new(x, y))].walkable
            }
        )
        {
            x = x + 1;
            if x == (map.width-1) as i32
            {
                y = y + 1;
                x = 1;
            }
        }
    }
    else
    {
        return pos;
    }

    Point::new(x, y)
}

pub fn create_player(ecs: &mut World, pos: Point)
{
    ecs.push
    (
        (
            pos,
            Renderable 
            {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK)
            },
            Player {},
            Viewshed
            {
                visible_tiles: HashSet::new(),
                range: 8,
                dirty: true
            }
        )
    );
}

pub fn spawn_klkan(ecs: &mut World, pos: Point)
{
    ecs.push
    (
        (
            pos,
            Renderable
            {
                glyph: rltk::to_cp437('K'),
                fg: RGB::named(rltk::BROWN1),
                bg: RGB::named(rltk::BLACK)
            },
            Enemy
            {
                health: 100
            }
        )
    );
}