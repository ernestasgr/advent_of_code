use std::{collections::HashSet, fs, time::Instant};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn movement(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (start_position, direction) = find_start(&mut grid);

    let duration = Instant::now();
    let visited_positions = simulate_guard_movement(&grid, start_position, direction);
    println!("Part 1 - Distinct positions visited: {}", visited_positions.len());
    println!("Part 1 - Duration: {:?}", duration.elapsed());

    let duration = Instant::now();
    let valid_obstruction_positions = find_valid_obstructions(&grid, start_position, direction);
    println!("Part 2 - Number of valid positions for obstruction: {}", valid_obstruction_positions.len());
    println!("Part 2 - Duration: {:?}", duration.elapsed());
}

fn find_start(grid: &mut [Vec<char>]) -> ((usize, usize), Direction) {
    let directions = [
        ('^', Direction::Up),
        ('v', Direction::Down),
        ('<', Direction::Left),
        ('>', Direction::Right),
    ];

    for (row, line) in grid.iter_mut().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if let Some(dir) = directions.iter().find_map(|&(c, d)| if ch == c { Some(d) } else { None }) {
                grid[row][col] = '.';
                return ((row, col), dir);
            }
        }
    }

    unreachable!("No start position found");
}

fn simulate_guard_movement(
    grid: &[Vec<char>],
    start_position: (usize, usize),
    start_direction: Direction,
) -> HashSet<(usize, usize)> {
    let mut position = start_position;
    let mut current_direction = start_direction;
    let mut visited = HashSet::new();

    visited.insert(position);

    while let Some(next_position) = next_position(&grid, position, current_direction.movement()) {
        match grid[next_position.0][next_position.1] {
            '#' | 'O' => current_direction = current_direction.turn_right(),
            _ => {
                position = next_position;
                visited.insert(position);
            }
        }
    }

    visited
}

fn find_valid_obstructions(
    grid: &[Vec<char>],
    start_position: (usize, usize),
    start_direction: Direction,
) -> HashSet<(usize, usize)> {
    let mut valid_positions = HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '.' && (row, col) != start_position {
                let mut test_grid = grid.to_vec();
                test_grid[row][col] = 'O';

                if is_guard_stuck(&test_grid, start_position, start_direction) {
                    valid_positions.insert((row, col));
                }
            }
        }
    }

    valid_positions
}

fn is_guard_stuck(
    grid: &[Vec<char>],
    start_position: (usize, usize),
    start_direction: Direction,
) -> bool {
    let mut position = start_position;
    let mut current_direction = start_direction;
    let mut visited_states = HashSet::new();

    while let Some(next_position) = next_position(&grid, position, current_direction.movement()) {
        if !visited_states.insert((position, current_direction)) {
            return true;
        }

        match grid[next_position.0][next_position.1] {
            '#' | 'O' => current_direction = current_direction.turn_right(),
            _ => position = next_position,
        }
    }

    false
}

fn next_position(
    grid: &[Vec<char>],
    position: (usize, usize),
    direction: (isize, isize),
) -> Option<(usize, usize)> {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let (x, y) = position;
    let (dx, dy) = direction;
    let next_x = x as isize + dx;
    let next_y = y as isize + dy;

    if next_x < 0 || next_y < 0 || next_x >= rows || next_y >= cols {
        None
    } else {
        Some((next_x as usize, next_y as usize))
    }
}
