use specs::prelude::*;
use specs_derive::Component;
use rltk::{ RGB, Point };

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32
}
impl Position {
    pub fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

#[derive(Component)]
pub struct RenderData {
    pub glyph: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub out_of_date: bool
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<RenderData>();
    world.register::<Player>();
    world.register::<Viewshed>();
}
