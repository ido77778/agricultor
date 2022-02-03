use std::cmp::{min, max};

use crate::prelude::*;

pub struct Rect
{
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32
}

impl Rect
{
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect
    {
        Rect
        {
            x1: x,
            x2: x+width,
            y1: y,
            y2: y+height
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool
    {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
    
    pub fn center(&self) -> (i32, i32)
    {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }
}

pub fn generate_dungeon(width: usize, height: usize) -> (Vec<u32>, Vec<Rect>)
{
    let mut map = vec![0; width * height]; // Creates a map array filled with stone walls.

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();
    
    for _ in 0..MAX_ROOMS
    {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, width as i32 - w - 1) - 1;
        let y = rng.roll_dice(1, height as i32 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter()
        {
            if new_room.intersect(other_room) { ok = false }
        }

        if ok
        {
            apply_room_to_map(&new_room, &mut map, width);

            if !rooms.is_empty()
            {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                if rng.range(0,2) == 1
                {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y, width, height);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x, width, height);
                }
                else
                {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x, width, height);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y, width, height);
                }
            }

            rooms.push(new_room);
        }
    }

    (map, rooms)
}

fn apply_room_to_map(room: &Rect, map: &mut Vec<u32>, width: usize)
{
    for y in room.y1 + 1 ..= room.y2
    {
        for x in room.x1 + 1 ..= room.x2
        {
            map[Point::new(x, y).to_index(width)] = 2; // Grass
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Vec<u32>, x1: i32, x2: i32, y: i32, width: usize, height: usize)
{
    for x in min(x1, x2) ..= max(x1, x2)
    {
        let id = Point::new(x, y).to_index(width);
        if id > 0 && id < width * height
        {
            map[id] = 2; // Grass.
        }
    }
}

fn apply_vertical_tunnel(map: &mut Vec<u32>, y1: i32, y2: i32, x: i32, width: usize, height: usize)
{
    for y in min(y1, y2) ..= max(y1, y2)
    {
        let id = Point::new(x, y).to_index(width);
        if id > 0 && id < width * height
        {
            map[id as usize] = 2; // Grass.
        }
    }
}