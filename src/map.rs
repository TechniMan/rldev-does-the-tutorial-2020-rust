use std::cmp::{ max, min };
use rltk::{ RGB, Rltk, RandomNumberGenerator };

use super::{ Rect };

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

const MAP_ARR_LENGTH: i32 = 4000;

/// Returns the value to index into an array of length 4000 (80*50)
pub fn xy_idx(x: i32, y: i32) -> i32 {
    (y * 80) + x
}

/// Makes an 80*50 map with solid boundaries and 400 randomly placed walls
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    // make the boundary walls
    for x in 0..80 {
        map[xy_idx(x, 0) as usize] = TileType::Wall;
        map[xy_idx(x, 49) as usize] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y) as usize] = TileType::Wall;
        map[xy_idx(79, y) as usize] = TileType::Wall;
    }

    // randomly splat a bunch of walls
    // thread-local RNG
    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        // don't block the player's starting cell
        if idx != xy_idx(40, 25) {
            map[idx as usize] = TileType::Wall;
        }
    }

    map
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < MAP_ARR_LENGTH {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < MAP_ARR_LENGTH {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1+1..=room.y2 {
        for x in room.x1+1..=room.x2 {
            map[xy_idx(x, y) as usize] = TileType::Floor;
        }
    }
}

/// Makes an 80*50 map with random rooms and connecting corridors
pub fn new_map_rooms_and_corridors() -> (Vec<TileType>, Vec<Rect>) {
    let mut map = vec![TileType::Wall; 80*50];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 5;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false;
                break;
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.centre();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].centre();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (map, rooms)
}

pub fn draw_map(map: &[TileType], context: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                context.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                context.set(x, y, RGB::from_f32(0., 1., 0.), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // move the 'cursor' along ot the next cell to draw
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
