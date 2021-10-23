use crate::state::State;
use crate::components::{Player, Position, Renderable, Viewshed};
use crate::map::{Map, HEIGHT, WIDTH};
use crate::JSON;

use rltk::{RGB, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::min;


pub fn try_move_player(delta_x: i32, delta_y: i32, gs: &mut State)
{
    // Changes the position of the player, if it is within limitations.
    let mut positions = gs.ecs.write_storage::<Position>();
    let mut players = gs.ecs.write_storage::<Player>();
    let map = gs.ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join()
    {
        // warn!("\nxyz_id: {}, get_tile: {}\nxyz_id + delta: {}, get_tile + delta: {}", &map.map_vector[xyz_id(pos.x, pos.y, *&map.current_level as i32)], &map.get_tile(pos.x, pos.y, *&map.current_level as i32), xyz_id(pos.x + delta_x, pos.y + delta_y, *&map.current_level as i32 + delta_z), &map.get_tile(pos.x + delta_x, pos.y + delta_y, *&map.current_level as i32 + delta_z));
        // warn!("current level: {}, pos.z: {}", &map.current_level, pos.z);
        let id = match &map.get_tile((pos.x + delta_x, pos.y + delta_y))
        {
            Some(id) => *id,
            None =>
            {
                error!("Player attempted to move unto a tile that doesn't exist - player::try_move_player");
                return()
            }
        };
        
        if JSON.with(|data| { data.tiles[&id].walkable }) == true
        {
            // Since we already handled the case of a negative value, we only need to handle
            // Values above map's dimensions.
            pos.x = min((WIDTH - 1) as i32, pos.x + delta_x);
            pos.y = min((HEIGHT - 1) as i32, pos.y + delta_y);
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk)
{
    // Handles player movement.
    match ctx.key
    {
        None => {} // Nothing happened.
        Some(key) => match key
        {
            // Directional Movement
            VirtualKeyCode::Left |
            VirtualKeyCode::H | 
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, gs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, gs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 | 
            VirtualKeyCode::K => try_move_player(0, -1, gs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, gs),

            // Sideways Movement
            VirtualKeyCode::Numpad1 => try_move_player(-1, 1, gs),
            VirtualKeyCode::Numpad3 => try_move_player(1, 1, gs),
            VirtualKeyCode::Numpad7 => try_move_player(-1, -1, gs),
            VirtualKeyCode::Numpad9 => try_move_player(1, -1, gs),
            _ => {}
        }
    }
}

pub fn create_player(gs: &mut State, x: i32, y: i32)
{    
    // Creates player.
    gs.ecs
      .create_entity()
      // Gives the player object its various components.
      .with(Position { x: x, y: y})
      .with(Renderable
        {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
      .with(Player {})
      .with(Viewshed { visible_tiles: Vec::new(), range: 8 })
      .build();
}
