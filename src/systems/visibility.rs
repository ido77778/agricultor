use crate::prelude::*;
use crate::map::Map;

#[system]
#[write_component(Viewshed)]
#[read_component(Point)]
pub fn visibility
(
    ecs: &mut SubWorld,
    #[resource] map: &mut Map,
)
{
    // TODO: This currently calculates only the players viewshed. This will have to change
    // when we have other entities with that component.
    let mut views = <(&Point, &mut Viewshed)>::query().filter(component::<Player>());
    views.iter_mut(ecs)
    .filter(|(_, view)| view.dirty) // Only recalculate dirty viewsheds.
    .for_each
    (
        |(pos, view)|
        {
            view.visible_tiles = field_of_view_set(*pos, view.range, map);
            view.dirty = false;

            for tile in view.visible_tiles.iter()
            {
                map.revealed_tiles[tile.to_index(map.width)] = true;
            }
        }
    )
    
}