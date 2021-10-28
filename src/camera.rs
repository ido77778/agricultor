use crate::prelude::*;

pub struct Camera
{
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32
}

impl Camera
{
    pub fn new(player_position: Point) -> Self
    {
        Self
        {
            left_x: player_position.x - (DISPLAY_WIDTH/2) as i32,
            right_x: player_position.x + (DISPLAY_WIDTH/2) as i32,
            top_y: player_position.x - (DISPLAY_HEIGHT/2) as i32,
            bottom_y: player_position.x + (DISPLAY_HEIGHT/2) as i32
        }
    }

    pub fn on_player_move(&mut self, new_position: &Point)
    {
        self.left_x = new_position.x - (DISPLAY_WIDTH/2) as i32;
        self.right_x = new_position.x + (DISPLAY_WIDTH/2) as i32;
        self.top_y = new_position.y - (DISPLAY_HEIGHT/2) as i32;
        self.bottom_y = new_position.y + (DISPLAY_HEIGHT/2) as i32;
    }
}