use crate::map::{Map};
use crate::JSON;

use rltk::{RGB, Rltk};

pub fn draw_level(map: &Map, ctx: &mut Rltk)
{
    // Indices
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while y < map.height as i32
    {
        // This should never panic.
        let tile = map.get_tile((x, y)).unwrap();

        let color = JSON.with(|data|{ data.tiles[&tile].rgb });
        let glyph = JSON.with(|data|{ data.tiles[&tile].glyph });
        // Drawing the tile.
        ctx.set(x, y, RGB::from_u8(color.0, color.1, color.2), RGB::from_f32(0., 0., 0.), rltk::to_cp437(glyph));
    
        // Advancing the loop forward.
        x += 1;
        if x > map.width as i32
        {
            x = 0;
            y += 1;
        }
    }
}
