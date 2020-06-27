use std::cmp::{ min, max };
use rltk::{ VirtualKeyCode as VKC, Rltk };
use specs::prelude::*;

use super::{ TransformData, Player, TileType, State, Map };

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut transforms = world.write_storage::<TransformData>();
    let mut players = world.write_storage::<Player>();
    let map = world.fetch::<Map>();

    for (_, transform) in (&mut players, &mut transforms).join() {
        let destination_idx = map.xy_idx(transform.x + delta_x, transform.y + delta_y);
        if map.tiles[destination_idx as usize] != TileType::Wall {
            transform.x = min(79, max(0, transform.x + delta_x));
            transform.y = min(49, max(0, transform.y + delta_y));
        }
    }
}

pub fn player_input(state: &mut State, context: &mut Rltk) {
    match context.key {
        None => {}
        Some(key) => match key {
            VKC::Left | VKC::Numpad4 | VKC::H =>
                try_move_player(-1, 0, &mut state.world),
            VKC::Right | VKC::Numpad6 | VKC::L =>
                try_move_player(1, 0, &mut state.world),
            VKC::Up | VKC::Numpad8 | VKC::K =>
                try_move_player(0, -1, &mut state.world),
            VKC::Down | VKC::Numpad2 | VKC::J =>
                try_move_player(0, 1, &mut state.world),
            VKC::Numpad1 =>
                try_move_player(-1, 1, &mut state.world),
            VKC::Numpad3 =>
                try_move_player(1, 1, &mut state.world),
            VKC::Numpad7 =>
                try_move_player(-1, -1, &mut state.world),
            VKC::Numpad9 =>
                try_move_player(1, -1, &mut state.world),
            _ => {}
        }
    }
}
