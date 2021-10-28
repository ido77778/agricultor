use rand::Rng;

use crate::prelude::*;

const DEATH_LIMIT: u8 = 3; // 3
const BIRTH_LIMIT: u8 = 4; // 4
const NUMBER_OF_ITERATIONS: u8 = 4; // 2-4
const CHANCE_TO_FLOOR: f32 = 0.4; // 0.4

pub fn generate_cavern(width: usize, height: usize) -> Vec<u32>
{
    let mut cave = vec![4 as u32; (width+1)*(height+1)];

    for tile in cave.iter_mut()
    {
        // Generating a random cave.
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.00..1.00) < CHANCE_TO_FLOOR
        {
            *tile = 5 as u32;
        }
    }
    
    for _ in (0..NUMBER_OF_ITERATIONS).step_by(1)
    {
        cave = iterate(cave, width, height);
    }

    cave
}

fn iterate(old_cave: Vec<u32>, width: usize, height: usize) -> Vec<u32>
{
    let mut new_cave = vec![0; height * width]; // Initializing a new vector.

    for x in (0..width).step_by(1)
    {
        for y in (0..height).step_by(1)
        {
            // warn!("({}, {})", x, y);

            let current_tile = Point::new(x, y).to_index(width);
            let alive_neighbour_count = count_alive_neighbours(&old_cave, (x as i32, y as i32), width, height);

            if old_cave[Point::new(x, y).to_index(width)] == 5 // If it's a floor,
            {
                if alive_neighbour_count < DEATH_LIMIT
                {
                    new_cave[current_tile] = 4; // Turn it into a wall.
                }
                else
                {
                    new_cave[current_tile] = 5 // Keep it a floor.
                }
            }
            else // If it's a wall,
            {
                if alive_neighbour_count > BIRTH_LIMIT
                {
                    new_cave[current_tile] = 5; // turn it into a floor.
                }
                else
                {
                    new_cave[current_tile] = 4; // Keep it a wall. 
                }
            }
        }
    }
    new_cave
}

fn count_alive_neighbours(cave: &Vec<u32>, tile: (i32, i32), width: usize, height: usize) -> u8
{
    // Counting the number of floor neighbours by iterating through them.
    let mut count: u8 = 0;
    for i in (-1..2).step_by(1)
    {
        for j in (-1..2).step_by(1) 
        {
            let neighbour_x = tile.0 + i;
            let neighbour_y = tile.1 + j;

            // warn!("tile {}, {}: ({}, {})", tile.0, tile.1, neighbour_x, neighbour_y);

            if neighbour_x < 0 || neighbour_y < 0 || neighbour_x >= width as i32 || neighbour_y >= height as i32
            {
                count = count + 1;
                // warn!("tile {}, {}: count {}", tile.0, tile.1, count);
            }
            else if cave[Point::new(neighbour_x, neighbour_y).to_index(width)] == 5
            {
                count = count + 1;
                // warn!("tile {}, {}: count {}", tile.0, tile.1, count);
            }
        }
    }

    // warn!("{}", count);
    count
}