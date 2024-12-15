use std::fs;
use std::collections::HashMap;
use image::{ImageBuffer, Rgb};

fn main() {
    let width = 101;
    let height = 103;

    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let robots: Vec<((isize, isize), (isize, isize))> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let pos = parse_pair(parts[0].strip_prefix("p=").unwrap_or(""));
                let vel = parse_pair(parts[1].strip_prefix("v=").unwrap_or(""));
                if let (Some(p), Some(v)) = (pos, vel) {
                    return Some((p, v));
                }
            }
            None
        })
        .collect();

    calculate_safety_factor(&robots, width, height);

    run_simulation(&robots, width, height, 10000);
}

fn calculate_safety_factor(robots: &Vec<((isize, isize), (isize, isize))>, width: usize, height: usize) {
    let time = 100;
    let mut final_positions = Vec::new();

    for (position, velocity) in robots {
        let (px, py) = *position;
        let (vx, vy) = *velocity;

        let new_x = (px + vx * time).rem_euclid(width as isize);
        let new_y = (py + vy * time).rem_euclid(height as isize);

        final_positions.push((new_x as usize, new_y as usize));
    }

    let mut tile_counts = HashMap::new();
    for position in final_positions {
        *tile_counts.entry(position).or_insert(0) += 1;
    }

    let mut quadrants = [0, 0, 0, 0];
    let mid_x = width / 2;
    let mid_y = height / 2;

    for ((x, y), count) in tile_counts {
        if x == mid_x || y == mid_y {
            continue; 
        }

        if x < mid_x && y < mid_y {
            quadrants[0] += count; 
        } else if x >= mid_x && y < mid_y {
            quadrants[1] += count; 
        } else if x < mid_x && y >= mid_y {
            quadrants[2] += count; 
        } else {
            quadrants[3] += count; 
        }
    }

    let safety_factor = quadrants.iter().product::<usize>();

    println!("Quadrants: {:?}", quadrants);
    println!("Safety Factor: {}", safety_factor);
}

fn run_simulation(
    robots: &Vec<((isize, isize), (isize, isize))>,
    width: usize,
    height: usize,
    iterations: usize,
) {
    let current_positions = robots.clone();

    for t in 0..iterations {
        let mut grid = vec![vec![0; width]; height];

        for (position, velocity) in &current_positions {
            let (px, py) = *position;
            let (vx, vy) = *velocity;

            let new_x = (px + vx * t as isize).rem_euclid(width as isize) as usize;
            let new_y = (py + vy * t as isize).rem_euclid(height as isize) as usize;

            grid[new_y][new_x] += 1;
        }

        save_grid_as_image(&grid, t);
    }
}

fn save_grid_as_image(grid: &Vec<Vec<usize>>, iteration: usize) {
    let width = grid[0].len() as u32;
    let height = grid.len() as u32;

    let mut img = ImageBuffer::new(width, height);

    for (y, row) in grid.iter().enumerate() {
        for (x, &count) in row.iter().enumerate() {
            let brightness = (count as u8).saturating_mul(50); 
            img.put_pixel(x as u32, y as u32, Rgb([brightness, brightness, brightness]));
        }
    }

    let filename = format!("output/output_{:05}.png", iteration);
    img.save(&filename).expect("Failed to save image");
    println!("Saved iteration {} as {}", iteration, filename);
}

fn parse_pair(input: &str) -> Option<(isize, isize)> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
            return Some((x, y));
        }
    }
    None
}
