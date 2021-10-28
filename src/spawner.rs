use std::collections::HashSet;

use crate::prelude::*;
use crate::{map::Map};

pub fn create_player(map: &Map, ecs: &mut World, pos: Point) -> Point
{
    let mut spawn_pos = Point::new(0, 0);
    if !JSON.with(|data| { data.tiles[map.get_tile(pos)].walkable })
    {
        // If the supplied tile isn't walkable, we iterate over the map until we find one that does.
        let mut x = 0;
        let mut y = 0;
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

        spawn_pos = Point::new(x, y);
    }
    else
    {
        spawn_pos = pos;
    }

    ecs.push
    (
        (
            spawn_pos,
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

    spawn_pos
}