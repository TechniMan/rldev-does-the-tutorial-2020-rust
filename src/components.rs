use specs::prelude::*;
use specs_derive::Component;
use rltk::{ RGB };

#[derive(Component)]
pub struct TransformData {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct RenderData {
    pub glyph: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB
}

#[derive(Component)]
pub struct Player {}
