use crate::prelude::*;
use crate::map::Map;
use crate::systems::build_scheduler;
use crate::camera::Camera;

pub struct State
{
    pub ecs: World,
    pub resources: Resources,
    pub systems: Schedule
}

impl GameState for State
{
    fn tick(&mut self, ctx: &mut Rltk)
    {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

impl State
{
    pub fn new() -> Self
    {
        let mut ecs = World::default();
        let mut resources = Resources::default();

        let map = Map::new();
        let player_position = Point::new(map.rooms[0].center().0, map.rooms[0].center().1);

        create_player(&mut ecs, player_position);
        resources.insert(Camera::new(player_position));

        
        for (i, room) in map.rooms.iter().skip(1).enumerate()
        {
            let (x, y) = room.center();
            spawn_klkan(&mut ecs, Point::new(x, y))
        }

        resources.insert(map);
        Self
        {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}
