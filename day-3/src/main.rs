use regex::Regex;
use std::{fs, time::Instant};

fn part_1(input: &str) -> i32 {
    let multiplication_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    multiplication_regex
        .captures_iter(input)
        .map(|capture| {
            let x: i32 = capture[1].parse().unwrap();
            let y: i32 = capture[2].parse().unwrap();
            x * y
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let multiplication_regex = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"^do\(\)").unwrap();
    let dont_regex = Regex::new(r"^don't\(\)").unwrap();

    let mut is_enabled = true;
    let mut total_sum = 0;
    let mut position = 0;
    
    while position < input.len() {
        let slice = &input[position..];

        if let Some(capture) = multiplication_regex.captures(slice) {
            if is_enabled {
                let x: i32 = capture[1].parse().unwrap();
                let y: i32 = capture[2].parse().unwrap();
                total_sum += x * y;
            }
            position += capture.get(0).unwrap().end();
        } else if let Some(mat) = do_regex.find(slice) {
            is_enabled = true;
            position += mat.end(); 
        } else if let Some(mat) = dont_regex.find(slice) {
            is_enabled = false;
            position += mat.end(); 
        } else {
            position += 1;
        }
    }

    total_sum
}

fn main() {
    let total_timer = Instant::now();

    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let part_1_timer = Instant::now();
    let part_1_sum = part_1(&input);
    println!("Part 1 time: {:?}", part_1_timer.elapsed());
    println!("Part 1 result: {:?}", part_1_sum);

    let part_2_timer = Instant::now();
    let part_2_sum = part_2(&input);
    println!("Part 2 time: {:?}", part_2_timer.elapsed());
    println!("Part 2 result: {:?}", part_2_sum);

    println!("Total time: {:?}", total_timer.elapsed());
}


