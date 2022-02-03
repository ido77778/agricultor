use legion::Schedule;

pub mod draw_map;
pub mod draw_entity;
pub mod player_input;
pub mod visibility;
pub mod collisions;

pub fn build_scheduler() -> Schedule
{
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(collisions::collisions_system())
    .add_system(visibility::visibility_system())
    .add_system(draw_map::draw_map_system())
    .add_system(draw_entity::draw_entity_system())
    .build()
}