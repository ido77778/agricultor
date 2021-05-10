use crate::state::State;
use crate::components::{Position, Player, Renderable};
use crate::map::xyz_id;

use rltk::{RGB, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};


pub fn try_move_player(delta_x: i32, delta_y: i32, delta_z: i32, gs: &mut State)
{
    // Changes the position of the player, if it is within limitations.
    let mut positions = gs.ecs.write_storage::<Position>();
    let mut players = gs.ecs.write_storage::<Player>();
    let map = gs.ecs.fetch::<Vec<u32>>();

    for (_player, pos) in (&mut players, &mut positions).join()
    {
        let destination_id = xyz_id(pos.x + delta_x, pos.y + delta_y, pos.z + delta_z);
        // TODO: Handle error if tile does not exist.
        if gs.tile_types.get(&map[destination_id]).unwrap().walkable == false
        {
            // If the would be new position is smaller than the minimum possible, use the minimum.
            // Otherwise, if it is bigger than the maximum, use the maximum.
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            pos.z = min(19, max(-10, pos.z + delta_z));
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
            // Arrow Keys
            VirtualKeyCode::Left => try_move_player(-1, 0, 0, &mut gs),
            VirtualKeyCode::Right => try_move_player(1, 0, 0, &mut gs),
            VirtualKeyCode::Up => try_move_player(0, -1, 0, &mut gs),
            VirtualKeyCode::Down => try_move_player(0, 1, 0, &mut gs),
            // Numpad
            VirtualKeyCode::Numpad1 => try_move_player(-1, 1, 0, &mut gs),
            VirtualKeyCode::Numpad2 => try_move_player(0, 1, 0, &mut gs),
            VirtualKeyCode::Numpad3 => try_move_player(1, 1, 0, &mut gs),
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, 0, &mut gs),
            VirtualKeyCode::Numpad6 => try_move_player(1, 0, 0, &mut gs),
            VirtualKeyCode::Numpad7 => try_move_player(-1, -1, 0, &mut gs),
            VirtualKeyCode::Numpad8 => try_move_player(0, -1, 0, &mut gs),
            VirtualKeyCode::Numpad9 => try_move_player(1, -1, 0, &mut gs),
            // Z-level Movement
            VirtualKeyCode::RBracket => try_move_player(0, 0, 1, &mut gs),
            VirtualKeyCode::LBracket => try_move_player(0, 0, -1, &mut gs),
            _ => {}
        }
    }
}

pub fn create_player(gs: &mut State, x: i32, y: i32, z: i32)
{    
    // Creates player.
    gs.ecs
      .create_entity()
      // Gives the player object its various components.
      .with(Position { x: x, y: y, z: z })
      .with(Renderable
        {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
      .with(Player {})
      .build();
}