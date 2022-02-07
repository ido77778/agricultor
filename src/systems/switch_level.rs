use crate::overmap::Overmap;
use crate::prelude::*;
use crate::map::Map;

#[system]
#[write_component(Point)]
pub fn switch_level
(
    ecs: &mut SubWorld,
    cb: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] overmap: &Overmap
)
{
    // Get player.
    let player_entity = *<Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    for e in Entity::query().iter(ecs)
    {
        if e != &player_entity
        {
            cb.remove(*e);
        }
    }

    
    
    
}