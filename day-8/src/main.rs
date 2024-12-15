use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let lines: Vec<&str> = input.lines().collect();
    let bounds_x = lines[0].len() as i32;
    let bounds_y = lines.len() as i32;
    let mut antennas: Vec<(i32, i32, char)> = Vec::with_capacity(bounds_x as usize * bounds_y as usize);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_alphanumeric() {
                antennas.push((x as i32, y as i32, ch));
            }
        }
    }

    let result_part1 = count_unique_antinodes(&antennas, bounds_x, bounds_y, false);
    println!("Part 1 - Unique antinode locations: {}", result_part1);

    let result_part2 = count_unique_antinodes(&antennas, bounds_x, bounds_y, true);
    println!("Part 2 - Unique antinode locations: {}", result_part2);
}

fn count_unique_antinodes(
    antennas: &[(i32, i32, char)],
    bounds_x: i32,
    bounds_y: i32,
    use_find_points: bool,
) -> usize {
    let mut antinodes = HashSet::new();

    for (i, (x1, y1, freq1)) in antennas.iter().enumerate() {
        for (_j, (x2, y2, freq2)) in antennas.iter().enumerate().skip(i + 1) {
            if freq1 == freq2 {
                let dx = x2 - x1;
                let dy = y2 - y1;

                if use_find_points {
                    let new_points = find_points_in_line(*x1, *y1, dx, dy, bounds_x, bounds_y);
                    for point in new_points {
                        antinodes.insert(point);
                    }
                } else {
                    let points_to_insert = [
                        (x1 + dx * 2, y1 + dy * 2),
                        (x2 - dx * 2, y2 - dy * 2),
                    ];
                    for point in &points_to_insert {
                        antinodes.insert(*point);
                    }
                }
            }
        }
    }

    antinodes.retain(|&(x, y)| x >= 0 && x < bounds_x && y >= 0 && y < bounds_y);
    antinodes.len()
}

fn find_points_in_line(
    x1: i32, y1: i32,
    dx: i32, dy: i32,
    bx: i32, by: i32
) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    points.extend(generate_points_in_direction(x1, y1, dx, dy, bx, by));
    points.extend(generate_points_in_direction(x1 - dx, y1 - dy, -dx, -dy, bx, by));

    points
}

fn generate_points_in_direction(
    x1: i32, y1: i32,
    dx: i32, dy: i32,
    bx: i32, by: i32
) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let mut x = x1;
    let mut y = y1;

    while x >= 0 && x <= bx && y >= 0 && y <= by {
        points.push((x, y));
        x += dx;
        y += dy;
    }

    points
}
