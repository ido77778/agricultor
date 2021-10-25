use crate::components::Player;
use super::{Viewshed, Position, Map};

use specs::prelude::*;
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem 
{
    type SystemData = (
                        WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>, 
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>
                      );

    fn run(&mut self, data: Self::SystemData)
    {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join()
        {
            if viewshed.dirty
            {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32 );
    
                // let viewshed_copy1 = viewshed.visible_tiles.clone();
                // let viewshed_copy2 = viewshed.visible_tiles.clone();
                // warn!("({}, {})", viewshed_copy1.into_iter().next().unwrap().x, viewshed_copy2.into_iter().next().unwrap().y);
    
                let p = player.get(ent);
                for tile in &mut map.visible_tiles { *tile = false } // Wipe visible tiles.
                if let Some(_p) = p
                {
                    for vis in viewshed.visible_tiles.iter() 
                    {
                        let idx = Map::xy_id((vis.x, vis.y));
                        map.revealed_tiles[idx] = true;
                        // visible_tiles is exactly the same as the player's viewshed. We reset it every time the player moves.
                        // We do this round-about process because in the renderer, using contain on the viewshed is much slower than
                        // looking up an index in visible tiles when we want to gray scale the tiles that aren't in the viewshed.
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}