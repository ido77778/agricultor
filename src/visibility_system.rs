use rltk::{Point, Point, field_of_view};
use specs::prelude::*;

use super::{Viewshed, Position, Map};

pub struct VisibilitySystem {}

pub struct Quadrant
{
    /* North = 0
       East  = 1 
       South = 2
       West  = 3 */
    cardinal: u16,
    origin: Point
}

impl<'a> System<'a> for VisibilitySystem
{
    type SystemData = (ReadExpect<'a, Map>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,);

    fn run(&mut self, data: Self::SystemData)
    {
        let (map, mut viewshed, pos) = data;

        for (viewshed, pos) in (&mut viewshed, &pos).join()
        {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map)
        }
    }
}

fn shdaow_casting_fov(start: Point, range: i32, map: Read<Map, PanicHandler>) -> Vec<Point>
{
    let visible_tiles = Vec::new();
    visible_tiles.push(start); // Mark the origin as visible.

    for i in 0..3
    {
        let quadrant = Quadrant { cardinal: i, origin: start };

        fn reveal(tile: Point)
        {
            visible_tiles.push()
        }
    }


}

impl Quadrant
{
    fn transform(self, tile: Point) -> Point
    {
        // Convert tile from the coordinates of the current quadrant
        // to absolute map coordinates.
        if self.cardinal == 0 { return Point::new(self.origin.x + tile.x, self.origin.y - tile.y); } // North
        if self.cardinal == 2 { return Point::new(self.cardinal.x + tile.x, self.origin.y + tile.y); } // South
        if self.cardinal == 1 { return Point::new(self.cardinal.x + tile.y, self.origin.y + tile.x); } // East
        if self.cardinal == 3 { return Point::new(self.cardinal.x - tile.y, self.origin.y + tile.x); } // West
    }
}