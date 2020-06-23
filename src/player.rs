use std::cmp::{ min, max };
use rltk::{ VirtualKeyCode, Rltk };
use specs::prelude::*;

use super::{ TransformData, Player, TileType, xy_idx, State };

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut transforms = world.write_storage::<TransformData>();
    let mut players = world.write_storage::<Player>();
    let map = world.fetch::<Vec<TileType>>();

    for (_, transform) in (&mut players, &mut transforms).join() {
        let destination_idx = xy_idx(transform.x + delta_x, transform.y + delta_y);
        if map[destination_idx as usize] != TileType::Wall {
            transform.x = min(79, max(0, transform.x + delta_x));
            transform.y = min(49, max(0, transform.y + delta_y));
        }
    }
}

pub fn player_input(state: &mut State, context: &mut Rltk) {
    match context.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut state.world),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut state.world),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut state.world),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut state.world),
            _ => {}
        }
    }
}
