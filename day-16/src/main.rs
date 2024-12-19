use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;
use std::fs;

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    position: (usize, usize),
    orientation: usize,
    score: usize,
    path: HashSet<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_maze(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                'S' => start = (y, x),
                'E' => end = (y, x),
                _ => {}
            }
        }
    }

    (grid, start, end)
}

fn find_lowest_score(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> (usize, usize) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut heap = BinaryHeap::new();
    let mut best_scores = HashMap::new();
    let mut best_score = usize::MAX;
    let mut unique_tiles = HashSet::new();
    unique_tiles.insert(start);

    heap.push(Reverse(State {
        position: start,
        orientation: 0,
        score: 0,
        path: HashSet::new(),
    }));

    while let Some(Reverse(state)) = heap.pop() {
        if state.score > best_score {
            continue;
        }

        if state.position == end {
            if state.score <= best_score {
                best_score = state.score;
                unique_tiles.extend(state.path.clone());
            }
            continue;
        }

        let state_key = (state.position.0, state.position.1, state.orientation);
        if let Some(&existing_score) = best_scores.get(&state_key) {
            if existing_score < state.score {
                continue;
            }
        }
        best_scores.insert(state_key, state.score);

        let (dy, dx) = directions[state.orientation];
        let new_pos = (
            (state.position.0 as isize + dy) as usize,
            (state.position.1 as isize + dx) as usize,
        );

        if maze[new_pos.0][new_pos.1] != '#' {
            let mut new_path = state.path.clone();
            new_path.insert(new_pos);
            heap.push(Reverse(State {
                position: new_pos,
                orientation: state.orientation,
                score: state.score + 1,
                path: new_path,
            }));
        }

        for &rotation in &[-1, 1] {
            let new_orientation = (state.orientation as isize + rotation).rem_euclid(4) as usize;
            heap.push(Reverse(State {
                position: state.position,
                orientation: new_orientation,
                score: state.score + 1000,
                path: state.path.clone(),
            }));
        }
    }

    (best_score, unique_tiles.len())
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (maze, start, end) = parse_maze(&input);
    let (score, unique_tile_count) = find_lowest_score(maze, start, end);

    println!("Lowest score: {}", score);
    println!("Unique tiles visited on best paths: {}", unique_tile_count);
}

