use std::cmp::{ max, min };
use specs::prelude::*;
use rltk::{ Rltk, RandomNumberGenerator, Point, Algorithm2D, BaseMap };
use super::{ Rect, COLOURS };

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles : Vec<TileType>,
    pub explored_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked_tiles: Vec<bool>,
    pub rooms : Vec<Rect>,
    pub width: i32,
    pub height: i32
}
impl Map {
    fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Returns the value to index into the tiles array
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.size() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.size() {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1+1..=room.y2 {
            for x in room.x1+1..=room.x2 {
                let idx = self.xy_idx(x, y) as usize;
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// Makes an 80*50 map with random rooms and connecting corridors
    pub fn new_map_rooms_and_corridors() -> Self {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80*50],
            explored_tiles: vec![false; 80*50],
            visible_tiles: vec![false; 80*50],
            blocked_tiles: vec![false; 80*50],
            rooms: Vec::new(),
            width: 80,
            height: 50
        };

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
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                    break;
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.centre();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].centre();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        // if out of bounds, return false straight away
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        // else, check if tile is blocked
        let idx = self.xy_idx(x, y);
        !self.blocked_tiles[idx]
    }

    /// Fill in the `map.blocked_tiles` vector
    pub fn populate_blocked(&mut self) {
        // for each tile, set to blocked if it is a wall
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked_tiles[i] = *tile == TileType::Wall;
        }
    }
}
impl BaseMap for Map {
    /// Returns whether tile at idx blocks movement
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        // find x and y for given tiles index
        let mut exits = rltk::SmallVec::new();
        let x = (idx as i32) % self.width;
        let y = (idx as i32) / self.width;
        let w = self.width as usize;

        // add each of cardinal directions which doesn't block
        if self.is_exit_valid(x-1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx - w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx + w, 1.0)) };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}
impl Algorithm2D for Map {
    /// Returns a Point containing the width & height of the map
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

/// RENDERING ///
/// ///////// ///
pub fn draw_map(world: &World, context: &mut Rltk) {
    let map = world.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.explored_tiles[idx] {
            let glyph;
            let fg;

            match tile {
                TileType::Floor => {
                    // glyph = rltk::to_cp437(rltk::to_char(176));
                    glyph = rltk::to_cp437('.');
                    fg = if map.visible_tiles[idx] { COLOURS[10] } else { COLOURS[12] }
                }
                TileType::Wall => {
                    // glyph = rltk::to_cp437(rltk::to_char(219));
                    glyph = rltk::to_cp437('#');
                    fg = if map.visible_tiles[idx] { COLOURS[9] } else { COLOURS[1] }
                }
            }
            context.set(x, y, fg, COLOURS[0], glyph);
        }

        // advance the 'cursor' along to the next cell to draw
        x += 1;
        if x > (map.width - 1) {
            x = 0;
            y += 1;
        }
    }
}
