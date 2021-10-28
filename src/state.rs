use crate::map::Map;
use crate::spawner::create_player;
use crate::systems::*;
use crate::camera::Camera;

use rltk::{GameState, Point, Rltk, render_draw_buffer};
use legion::*;

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
        let player_position = create_player(&map, &mut ecs, Point::new(45, 45));
        resources.insert(map);
        
        resources.insert(Camera::new(player_position));

        Self
        {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}
