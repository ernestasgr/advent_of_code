use std::{fs, time::Instant};

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report
        .windows(2)
        .map(|r| r[1] - r[0])
        .collect();
    let all_increasing = diffs
        .iter()
        .all(|&d| d >= 1 && d <= 3);
    let all_decreasing = diffs
        .iter()
        .all(|&d| d <= -1 && d >= -3);
    
    all_increasing || all_decreasing
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn main() {
    let start_total = Instant::now();

    let input_start = Instant::now();
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|x| x
                .parse::<i32>()
                .unwrap())
            .collect())
        .collect();
    let input_duration = input_start.elapsed();
    println!("Input parsing time: {:?}", input_duration);

    let part1_start = Instant::now();
    let safe_count = reports
        .iter()
        .filter(|report| is_safe(report))
        .count();
    let part1_duration = part1_start.elapsed();
    println!("Part 1: Safe reports: {}", safe_count);
    println!("Part 1 time: {:?}", part1_duration);

    let part2_start = Instant::now();
    let safe_with_dampener_count = reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();
    let part2_duration = part2_start.elapsed();
    println!("Part 2: Safe reports with dampener: {}", safe_with_dampener_count);
    println!("Part 2 time: {:?}", part2_duration);

    let total_duration = start_total.elapsed();
    println!("Total execution time: {:?}", total_duration);
}
