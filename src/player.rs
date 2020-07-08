use std::cmp::{ min, max };
use rltk::{ VirtualKeyCode as VKC, Rltk, Point };
use specs::prelude::*;

use super::{ Position, Player, Viewshed, State, Map };

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut players = world.write_storage::<Player>();
    let mut positions = world.write_storage::<Position>();
    let mut viewsheds = world.write_storage::<Viewshed>();
    let map = world.fetch::<Map>();

    for (_, position, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(position.x + delta_x, position.y + delta_y);
        if !map.blocked_tiles[destination_idx as usize] {
            // actually cause movement
            position.x = min(79, max(0, position.x + delta_x));
            position.y = min(49, max(0, position.y + delta_y));
            viewshed.out_of_date = true;

            let mut player_pos = world.write_resource::<Point>();
            player_pos.x = position.x;
            player_pos.y = position.y;
        }
    }
}

/// Returns true if input consumes player turn
pub fn player_input(state: &mut State, context: &mut Rltk) -> bool {
    match context.key {
        None => { return false; }
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
            _ => { return false; }
        }
    }
    true
}
