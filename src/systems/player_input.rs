use rltk::VirtualKeyCode;

use crate::camera::Camera;
use crate::prelude::*;
use crate::map::Map;

#[system]
#[write_component(Point)]
#[write_component(Viewshed)]
#[read_component(Player)]
pub fn player_input
(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
)
{
    if let Some(key) = key
    {
        let delta = match key
        {
                      // Directional Movement
                      VirtualKeyCode::Left |
                      VirtualKeyCode::A | 
                      VirtualKeyCode::Numpad4 => Point::new(-1, 0),
          
                      VirtualKeyCode::Right |
                      VirtualKeyCode::Numpad6 |
                      VirtualKeyCode::D => Point::new(1, 0),
          
                      VirtualKeyCode::Up |
                      VirtualKeyCode::Numpad8 | 
                      VirtualKeyCode::W => Point::new(0, -1),
          
                      VirtualKeyCode::Down |
                      VirtualKeyCode::Numpad2 |
                      VirtualKeyCode::S => Point::new(0, 1),
          
                      // Sideways Movement
                      VirtualKeyCode::Numpad1 => Point::new(-1, 1),
                      VirtualKeyCode::Numpad3 => Point::new(1, 1),
                      VirtualKeyCode::Numpad7 => Point::new(-1, -1),
                      VirtualKeyCode::Numpad9 => Point::new(1, -1),

                      // If the key isn't a movment key.
                      _ => Point::new(0, 0)
        };

        if delta.x != 0 || delta.y != 0
        {
            // Only retrieve the position of the player.
            let mut player = <(&mut Point, &mut Viewshed)>::query().filter(component::<Player>());
            player.iter_mut(ecs).for_each
            (
                |player|
                {
                    let destination = *player.0 + delta;
                    if map.can_player_walk(destination)
                    {
                        *player.0 = destination;
                        camera.on_player_move(&destination);
                        player.1.dirty = true;
                        *turn_state = TurnState::PlayerTurn;
                    }
                }
            );
        }
    }
}