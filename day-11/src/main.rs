use std::collections::HashMap;

fn split_number(n: u64) -> (u64, u64) {
    let digits = n.to_string();
    let mid = digits.len() / 2;
    let left: u64 = digits[..mid].parse().unwrap_or(0);
    let right: u64 = digits[mid..].parse().unwrap_or(0);
    (left, right)
}

fn simulate(initial_stones: Vec<u64>, blinks: usize) -> usize {
    let mut current_state: HashMap<u64, usize> = HashMap::new();

    for stone in initial_stones {
        *current_state.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut next_state: HashMap<u64, usize> = HashMap::new();

        for (&stone, &count) in &current_state {
            match stone {
                0 => {
                    *next_state.entry(1).or_insert(0) += count;
                }
                _ if stone.to_string().len() % 2 == 0 => {
                    let (left, right) = split_number(stone);
                    *next_state.entry(left).or_insert(0) += count;
                    *next_state.entry(right).or_insert(0) += count;
                }
                _ => {
                    *next_state.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }

        current_state = next_state;
    }

    current_state.values().sum()
}

fn main() {
    let initial_stones = vec![8793800, 1629, 65, 5, 960, 0, 138983, 85629];
    let total_blinks = 75;

    let result = simulate(initial_stones, total_blinks);
    
    println!("Number of stones after {} blinks: {}", total_blinks, result);
}