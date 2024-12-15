use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let map = read_map_from_file("input.txt");
    let total_score = sum_trailhead_scores(&map);
    let total_rating = sum_trailhead_ratings(&map);

    println!("Sum of trailhead scores: {}", total_score);
    println!("Sum of trailhead ratings: {}", total_rating);
}

fn read_map_from_file(file_path: &str) -> Vec<Vec<u8>> {
    let input = read_to_string(file_path).unwrap();

    input
        .lines()
        .map(|line| {
            line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect()
}

fn sum_trailhead_scores(map: &[Vec<u8>]) -> usize {
    let mut total_score = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                total_score += trailhead_score(map, i, j);
            }
        }
    }

    total_score
}

fn trailhead_score(map: &[Vec<u8>], start_row: usize, start_col: usize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut reachable_nines = HashSet::new();

    queue.push_back((start_row, start_col, 0));

    while let Some((row, col, height)) = queue.pop_front() {
        if !visited.insert((row, col)) {
            continue;
        }

        if map[row][col] == 9 {
            reachable_nines.insert((row, col));
            continue;
        }

        for (next_row, next_col) in neighbors(row, col, map.len(), map[0].len()) {
            if map[next_row][next_col] == height + 1 {
                queue.push_back((next_row, next_col, map[next_row][next_col]));
            }
        }
    }

    reachable_nines.len()
}

fn sum_trailhead_ratings(map: &[Vec<u8>]) -> usize {
    let mut total_rating = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                total_rating += trailhead_rating(map, i, j);
            }
        }
    }

    total_rating
}

fn trailhead_rating(map: &[Vec<u8>], start_row: usize, start_col: usize) -> usize {
    let mut memo = vec![vec![None; map[0].len()]; map.len()];
    dfs_count_paths(map, start_row, start_col, 0, &mut memo)
}

fn dfs_count_paths(
    map: &[Vec<u8>],
    row: usize,
    col: usize,
    height: u8,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if map[row][col] == 9 {
        return 1;
    }

    if let Some(cached) = memo[row][col] {
        return cached;
    }

    let mut path_count = 0;

    for (next_row, next_col) in neighbors(row, col, map.len(), map[0].len()) {
        if map[next_row][next_col] == height + 1 {
            path_count += dfs_count_paths(map, next_row, next_col, height + 1, memo);
        }
    }

    memo[row][col] = Some(path_count);
    path_count
}

fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if row > 0 {
        result.push((row - 1, col));
    }
    if row + 1 < rows {
        result.push((row + 1, col));
    }
    if col > 0 {
        result.push((row, col - 1));
    }
    if col + 1 < cols {
        result.push((row, col + 1));
    }

    result
}

