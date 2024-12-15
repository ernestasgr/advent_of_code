use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut total_calibration = 0;

    for line in input.lines() {
        if let Some((target, numbers)) = parse_line(line.trim()) {
            if evaluate_with_operators(target, &numbers, 0, numbers[0]) {
                total_calibration += target;
            }
        }
    }

    println!("Total calibration result: {}", total_calibration);
}

fn parse_line(line: &str) -> Option<(i64, Vec<i64>)> {
    if let Some((target, numbers)) = line.split_once(": ") {
        let target = target.parse::<i64>().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        Some((target, numbers))
    } else {
        None
    }
}

fn evaluate_with_operators(target: i64, numbers: &[i64], index: usize, current: i64) -> bool {
    if current > target {
        return false;
    }
    if index == numbers.len() - 1 {
        return current == target;
    }

    let next_number = numbers[index + 1];
    evaluate_with_operators(target, numbers, index + 1, current + next_number)
        || evaluate_with_operators(target, numbers, index + 1, current * next_number)
        || evaluate_with_operators(target, numbers, index + 1, (current.to_string() + &next_number.to_string()).parse::<i64>().unwrap())
}
