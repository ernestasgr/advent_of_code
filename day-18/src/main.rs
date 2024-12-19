use std::collections::{HashSet, VecDeque};
use std::fs;

fn read_input(file_path: &str) -> Vec<(usize, usize)> {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|x| x.parse().expect("Invalid number"));
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn initialize_corrupted(bytes: &[(usize, usize)], limit: usize) -> HashSet<(usize, usize)> {
    bytes.iter().take(limit).cloned().collect()
}

fn bfs(
    grid_size: usize,
    corrupted: &HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            return Some(steps);
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid_size
                && (ny as usize) < grid_size
                && !corrupted.contains(&(nx as usize, ny as usize))
                && !visited.contains(&(nx as usize, ny as usize))
            {
                visited.insert((nx as usize, ny as usize));
                queue.push_back(((nx as usize, ny as usize), steps + 1));
            }
        }
    }

    None
}

fn find_first_blocking_byte(
    grid_size: usize,
    bytes: &[(usize, usize)],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, usize)> {
    let mut corrupted = HashSet::new();

    for &(x, y) in bytes {
        corrupted.insert((x, y));
        if bfs(grid_size, &corrupted, start, end).is_none() {
            return Some((x, y));
        }
    }

    None
}

fn main() {
    let bytes = read_input("input.txt");
    let grid_size = 71;

    let corrupted = initialize_corrupted(&bytes, 1024);
    let start = (0, 0);
    let end = (grid_size - 1, grid_size - 1);

    match bfs(grid_size, &corrupted, start, end) {
        Some(steps) => println!("Minimum steps to reach the exit: {}", steps),
        None => println!("No path to the exit found!"),
    }

    match find_first_blocking_byte(grid_size, &bytes, start, end) {
        Some((x, y)) => println!("First blocking byte: {},{}", x, y),
        None => println!("All paths remain open."),
    }
}