use specs_derive::Component;
use specs::prelude::*;
use rltk::RGB;

pub struct Position
{
    pub x: i32,
    pub y: i32
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

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool
}

