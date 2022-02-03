use std::collections::HashSet;
use rltk::RGB;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable
{
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {}

#[derive(Clone, Debug, PartialEq)]
pub struct Viewshed
{
    pub visible_tiles: HashSet<rltk::Point>,
    pub range: i32,
    pub dirty: bool
}

impl Viewshed
{
    pub fn _new(range: i32) -> Self
    {
        Self
        {
            visible_tiles: HashSet::new(),
            range,
            dirty: true
        }
    }    
}

pub struct Enemy
{
    pub health: u16
}