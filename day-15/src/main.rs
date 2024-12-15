use std::{collections::VecDeque, fs::read_to_string};

#[derive(PartialEq, Clone, Copy)]
enum TileType {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

fn parse_tile_type(tile_char: char) -> TileType {
    match tile_char {
        '.' | '@' => TileType::Empty,
        '#' => TileType::Wall,
        'O' => TileType::BoxLeft,
        _ => panic!("Invalid tile character"),
    }
}

fn parse_tile_type_scaled(tile_char: char) -> [TileType; 2] {
    match tile_char {
        '.' | '@' => [TileType::Empty; 2],
        '#' => [TileType::Wall; 2],
        'O' => [TileType::BoxLeft, TileType::BoxRight],
        _ => panic!("Invalid tile character"),
    }
}

fn parse_move_direction(move_command: char, map_width: usize) -> isize {
    match move_command {
        '^' => -(map_width as isize),
        'v' => map_width as isize,
        '<' => -1,
        '>' => 1,
        _ => panic!("Invalid movement command"),
    }
}

fn calculate_box_gps_coordinate(pos: usize, map_width: usize) -> usize {
    pos % map_width + pos / map_width * 100
}

fn create_map(warehouse_map: &str, scale: usize) -> (Vec<TileType>, usize, isize) {
    let mut map_width = 0;
    let mut robot_position = 0;

    let map: Vec<_> = warehouse_map
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row_index, line)| {
            map_width = line.len() * scale;
            if let Some(col_index) = line.chars().position(|c| c == '@') {
                robot_position = col_index as isize * scale as isize + row_index as isize * map_width as isize;
            }
            if scale == 1 {
                line.chars().map(|c| parse_tile_type(c)).collect::<Vec<TileType>>().into_iter()
            } else {
                line.chars().flat_map(|c| parse_tile_type_scaled(c)).collect::<Vec<TileType>>().into_iter() 
            }
        })
        .collect();

    (map, map_width, robot_position)
}


fn execute_moves_part_1(
    map: &mut Vec<TileType>,
    map_width: usize,
    robot_position: &mut isize,
    move_sequence: &str,
) {
    for move_command in move_sequence.chars() {
        let move_direction = parse_move_direction(move_command, map_width);
        let mut new_robot_position = *robot_position;
        loop {
            new_robot_position += move_direction;
            match map[new_robot_position as usize] {
                TileType::Empty => {
                    *robot_position += move_direction;
                    map[new_robot_position as usize] = map[*robot_position as usize];
                    map[*robot_position as usize] = TileType::Empty;
                    break;
                }
                TileType::Wall => break,
                TileType::BoxLeft => {}
                TileType::BoxRight => panic!("Invalid box configuration"),
            }
        }
    }
}

fn collect_box_coordinates(map: Vec<TileType>, map_width: usize) -> usize {
    map
        .into_iter()
        .enumerate()
        .filter_map(|(pos, tile)| match tile {
            TileType::BoxLeft => Some(calculate_box_gps_coordinate(pos, map_width)),
            _ => None,
        })
        .sum()
}

fn part_1(warehouse_map: &str) -> usize {
    let (mut map, map_width, mut robot_position) = create_map(warehouse_map, 1);

    for move_sequence in warehouse_map.lines().skip_while(|line| !line.is_empty()).skip(1) {
        execute_moves_part_1(&mut map, map_width, &mut robot_position, move_sequence);
    }

    collect_box_coordinates(map, map_width)
}

fn execute_moves_part_2(
    map: &mut Vec<TileType>,
    map_width: usize,
    robot_position: &mut isize,
    move_sequence: &str,
) {
    let mut boxes_to_move = Vec::new();
    let mut positions_to_check = VecDeque::new();

    for move_command in move_sequence.chars() {
        let move_direction = parse_move_direction(move_command, map_width);
        let mut is_movement_blocked = false;
        positions_to_check.push_back(*robot_position + move_direction);

        while let Some(position_to_check) = positions_to_check.pop_front() {
            if let Some(box_start_position) = match map[position_to_check as usize] {
                TileType::Empty => None,
                TileType::Wall => {
                    is_movement_blocked = true;
                    break;
                }
                TileType::BoxLeft => Some(position_to_check),
                TileType::BoxRight => Some(position_to_check - 1),
            } {
                if !boxes_to_move.contains(&box_start_position) {
                    boxes_to_move.push(box_start_position);
                    if move_direction != 1 {
                        positions_to_check.push_back(box_start_position + move_direction);
                    }
                    if move_direction != -1 {
                        positions_to_check.push_back(box_start_position + 1 + move_direction);
                    }
                }
            }
        }

        if !is_movement_blocked {
            *robot_position += move_direction;
            for &box_start_position in boxes_to_move.iter().rev() {
                map[box_start_position as usize] = TileType::Empty;
                map[box_start_position as usize + 1] = TileType::Empty;
                map[(box_start_position + move_direction) as usize] = TileType::BoxLeft;
                map[(box_start_position + 1 + move_direction) as usize] = TileType::BoxRight;
            }
        }

        boxes_to_move.clear();
        positions_to_check.clear();
    }
}

fn part_2(warehouse_map: &str) -> usize {
    let (mut map, map_width, mut robot_position) = create_map(warehouse_map, 2);

    for move_sequence in warehouse_map.lines().skip_while(|line| !line.is_empty()).skip(1) {
        execute_moves_part_2(&mut map, map_width, &mut robot_position, move_sequence);
    }

    collect_box_coordinates(map, map_width)
}

fn main() {
    let warehouse_input = read_to_string("input.txt").unwrap();

    let part_1_result = part_1(&warehouse_input);
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = part_2(&warehouse_input);
    println!("Part 2 result: {}", part_2_result);
}
