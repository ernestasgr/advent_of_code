use std::{fs::read_to_string, io, time::Instant};

fn main() -> io::Result<()> {
    let input: Vec<String> = read_to_string("input.txt")?
		.lines()
		.map(|s| s.to_string())
		.collect();
    let grid: Vec<Vec<char>> = input
		.iter()
		.map(|line| line.chars().collect())
		.collect();

    let start_part1 = Instant::now();
    let part1_count = part1(&grid, "XMAS");
    let duration_part1 = start_part1.elapsed();
    println!("Part 1: XMAS appears {} times", part1_count);
    println!("Part 1 completed in {:?}", duration_part1);

    let start_part2 = Instant::now();
    let part2_count = part2(&grid);
    let duration_part2 = start_part2.elapsed();
    println!("Part 2: X-MAS appears {} times", part2_count);
    println!("Part 2 completed in {:?}", duration_part2);

    Ok(())
}

fn part1(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let word_length = word.len();
    let word: Vec<char> = word.chars().collect();

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];

    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut matched = true;
                for i in 0..word_length {
                    let new_row = row + i as isize * dr;
                    let new_col = col + i as isize * dc;

                    if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
                        matched = false;
                        break;
                    }

                    if grid[new_row as usize][new_col as usize] != word[i] {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows - 2 {
        for col in 0..cols - 2 {
            if is_x_mas(grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_x_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let top_left_mas = [
        grid[row][col],
        grid[row + 1][col + 1],
        grid[row + 2][col + 2],
    ];
    let bottom_left_mas = [
        grid[row + 2][col],
        grid[row + 1][col + 1],
        grid[row][col + 2],
    ];

    let valid_mas = |mas: &[char]| mas == ['M', 'A', 'S'] || mas == ['S', 'A', 'M'];

    valid_mas(&top_left_mas) && valid_mas(&bottom_left_mas)
}
