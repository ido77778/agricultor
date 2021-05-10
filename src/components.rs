use specs_derive::Component;
use specs::prelude::*;
use rltk::RGB;

pub struct Position
{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Component for Position
{
    type Storage = VecStorage<Self>;
}

#[derive(Component)]
pub struct Renderable
{
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
#[derive(Component, Debug)]
pub struct Player {}