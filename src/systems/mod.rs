use legion::Schedule;

pub mod draw_map;
pub mod draw_entity;
pub mod player_input;
pub mod visibility;
pub mod collision;
pub mod end_turn;
pub mod switch_level;

// pub fn build_scheduler() -> Schedule
// {
//     Schedule::builder()
//     .add_system(player_input::player_input_system())
//     .add_system(collision::collision_system())
//     .add_system(visibility::visibility_system())
//     .add_system(draw_map::draw_map_system())
//     .add_system(draw_entity::draw_entity_system())
//     .build()
// }

pub fn build_input_scheduler() -> Schedule
{
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(draw_map::draw_map_system())
    .add_system(draw_entity::draw_entity_system())
    .add_system(visibility::visibility_system())
    .flush()
    .build()
}

pub fn build_player_scheduler() -> Schedule
{
    Schedule::builder()
    .add_system(collision::collision_system())
    .flush()
    .add_system(draw_map::draw_map_system())
    .add_system(draw_entity::draw_entity_system())
    .add_system(visibility::visibility_system())
    .flush()
    .add_system(end_turn::end_turn_system())
    .build()
}

pub fn build_switch_level_scheduler() -> Schedule
{
    Schedule::builder()
    .add_system(switch_level::switch_level_system())
    .flush()
    .add_system(draw_map::draw_map_system())
    .add_system(draw_entity::draw_entity_system())
    .add_system(visibility::visibility_system())
    .flush()
    .add_system(end_turn::end_turn_system())
    .build()
}