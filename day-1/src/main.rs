use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn total_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    let start = Instant::now();

    left.sort_unstable();
    right.sort_unstable();

    let distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Time taken for total_distance: {:?}", start.elapsed());
    distance
}

fn similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let start = Instant::now();

    let mut right_counts = HashMap::new();
    for num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let score: i32 = left
        .iter()
        .map(|num| num * right_counts.get(num).unwrap_or(&0))
        .sum();

    println!("Time taken for similarity_score: {:?}", start.elapsed());
    score
}

fn main() -> io::Result<()> {
    let overall_start = Instant::now();

    let path = "input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if numbers.len() == 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }

    let distance = total_distance(&mut left, &mut right);
    let score = similarity_score(left, right);

    println!("Total distance: {}", distance);
    println!("Similarity score: {}", score);
    println!("Total execution time: {:?}", overall_start.elapsed());

    Ok(())
}
