use std::{collections::HashSet, fs};

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_regions(grid: &[Vec<char>]) -> Vec<(char, usize, Vec<(usize, usize)>)> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for y in 0..rows {
        for x in 0..cols {
            if !visited[y][x] {
                let plant = grid[y][x];
                let mut stack = vec![(y, x)];
                let mut cells = Vec::new();
                let mut perimeter = 0;

                while let Some((curr_y, curr_x)) = stack.pop() {
                    if visited[curr_y][curr_x] {
                        continue;
                    }
                    visited[curr_y][curr_x] = true;
                    cells.push((curr_x, curr_y)); 

                    for &(dx, dy) in &directions {
                        let (next_x, next_y) = (curr_x as isize + dx, curr_y as isize + dy);
                        if next_x >= 0 && next_y >= 0 && (next_x as usize) < cols && (next_y as usize) < rows {
                            let (next_x, next_y) = (next_x as usize, next_y as usize);
                            if grid[next_y][next_x] == plant && !visited[next_y][next_x] {
                                stack.push((next_y, next_x)); 
                            } else if grid[next_y][next_x] != plant {
                                perimeter += 1;
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }

                regions.push((plant, perimeter, cells));
            }
        }
    }

    regions
}

fn count_region_sides(region: &[(usize, usize)]) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let region_set: HashSet<_> = region.iter().copied().collect();
    let mut side_count = 0;

    for &(dx, dy) in &directions {
        let mut sides = HashSet::new();

        for &(x, y) in region {
            let neighbor = (x as isize + dx, y as isize + dy);
            if !region_set.contains(&(neighbor.0 as usize, neighbor.1 as usize)) {
                sides.insert(neighbor);
            }
        }

        let mut neighbors_to_remove = HashSet::new();
        for &side in &sides {
            let mut neighbor = (side.0 + dy, side.1 + dx);
            while sides.contains(&neighbor) {
                neighbors_to_remove.insert(neighbor);
                neighbor = (neighbor.0 + dy, neighbor.1 + dx);
            }
        }

        side_count += sides.len() - neighbors_to_remove.len();
    }

    side_count
}

fn calculate_total_cost<F>(regions: &[(char, usize, Vec<(usize, usize)>)], cost_fn: F) -> usize
where
    F: Fn(&(char, usize, Vec<(usize, usize)>)) -> usize,
{
    regions.iter().map(cost_fn).sum()
}

fn region_cost_part_1(region: &(char, usize, Vec<(usize, usize)>)) -> usize {
    let (_, perimeter, cells) = region;
    cells.len() * perimeter
}

fn region_cost_part_2(region: &(char, usize, Vec<(usize, usize)>)) -> usize {
    let (_, _, cells) = region;
    count_region_sides(cells) * cells.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let grid = parse_map(&input);
    let regions = find_regions(&grid);

    let total_cost_part_1 = calculate_total_cost(&regions, region_cost_part_1);
    println!("Total cost of fencing all regions (Part 1): {}", total_cost_part_1);

    let total_cost_part_2 = calculate_total_cost(&regions, region_cost_part_2);
    println!("Total cost of fencing all regions (Part 2): {}", total_cost_part_2);
}