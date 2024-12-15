use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let disk_map = read_to_string("input.txt").expect("Failed to read file");
    let mut blocks = parse_disk_map(disk_map);
    let is_part_2 = true;
    
    if is_part_2 {
        compact_disk_whole_files(&mut blocks);
    } else {
        compact_disk(&mut blocks);
    }
    
    let checksum = calculate_checksum(&blocks);
    
    println!("Checksum - {}", checksum);
}

fn parse_disk_map(disk_map: String) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current_file_id = 0;

    for (i, ch) in disk_map.chars().enumerate() {
        let length = ch.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            for _ in 0..length {
                blocks.push(current_file_id.to_string());
            }
            current_file_id += 1;
        } else {
            for _ in 0..length {
                blocks.push(".".to_string());
            }
        }
    }

    blocks
}

fn compact_disk(blocks: &mut Vec<String>) {
    for i in (0..blocks.len()).rev() {
        if blocks[i] != "." {
            let mut target_index = None;

            for j in 0..i {
                if blocks[j] == "." {
                    target_index = Some(j);
                    break;
                }
            }

            if let Some(target) = target_index {
                blocks[target] = blocks[i].clone();
                blocks[i] = ".".to_string();
            }
        }
    }
}

fn calculate_checksum(blocks: &[String]) -> u64 {
    blocks.iter().enumerate().fold(0, |checksum, (pos, block)| {
        if block != "." {
            checksum + pos as u64 * block.parse::<u64>().unwrap()
        } else {
            checksum
        }
    })
}

fn find_free_spaces(blocks: &[String]) -> Vec<(usize, usize)> {
    let mut free_spaces = Vec::new();
    let mut start = None;

    for (i, block) in blocks.iter().enumerate() {
        if block == "." {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(start_index) = start {
            free_spaces.push((start_index, i - 1));
            start = None;
        }
    }

    if let Some(start_index) = start {
        free_spaces.push((start_index, blocks.len() - 1));
    }

    free_spaces
}

fn compact_disk_whole_files(blocks: &mut Vec<String>) {
    let mut files: Vec<(String, Vec<usize>)> = Vec::new();
    let mut seen_files = HashSet::new();

    for (_i, block) in blocks.iter().enumerate() {
        if block != "." && !seen_files.contains(block) {
            let indices: Vec<usize> = blocks.iter().enumerate()
                .filter(|(_, b)| *b == block)
                .map(|(index, _)| index)
                .collect();
            files.push((block.clone(), indices));
            seen_files.insert(block.clone());
        }
    }

    files.sort_by(|a, b| b.0.parse::<i32>().unwrap().cmp(&a.0.parse::<i32>().unwrap()));

    for (file_id, indices) in files {
        let file_size = indices.len();
        let free_spaces = find_free_spaces(blocks);

        if let Some((start, _end)) = free_spaces.iter()
            .find(|(start, end)| end - start + 1 >= file_size && *end < indices[0]) 
        {
            let start = *start;

            for i in start..(start + file_size) {
                blocks[i] = file_id.clone();
            }

            for index in indices {
                blocks[index] = ".".to_string();
            }
        }
    }
}