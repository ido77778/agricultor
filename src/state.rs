use crate::overmap::Overmap;
use crate::{prelude::*, locations};
use crate::camera::Camera;

pub struct State
{
    pub ecs: World,
    pub resources: Resources,
    pub input_systems: Schedule,
    pub player_systems: Schedule,
    pub switch_level_systems: Schedule
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

        // Defines which sequence of systems to run on each state.
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state
        {
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::SwitchLevel => self.switch_level_systems.execute(&mut self.ecs, &mut self.resources)
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

impl State
{
    pub fn new() -> Self
    {
        let mut ecs = World::default();
        let mut resources = Resources::default();

        let map = locations::generate_new_map();
        let mut overmap = Overmap::new();
        let player_position = Point::new(map.rooms[0].center().0, map.rooms[0].center().1);

        // warn!("before: {}, {}", player_position.x, player_position.y);
        create_player(&mut ecs, player_position);
        resources.insert(Camera::new(player_position));

        
        for (_i, room) in map.rooms.iter().skip(1).enumerate()
        {
            let (x, y) = room.center();
            spawn_klkan(&mut ecs, Point::new(x, y))
        }

        resources.insert(TurnState::AwaitingInput);
        &mut overmap.store_map(&map);
        resources.insert(map);
        resources.insert(overmap);
        Self
        {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            switch_level_systems: build_switch_level_scheduler()
        }
    }
}
