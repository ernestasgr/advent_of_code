use std::{collections::{HashMap, HashSet}, fs::File, io::{self, BufRead}};


fn read_rules_and_updates(file_path: &str) -> io::Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains('|') {
            let parts: Vec<i32> = line
                .split('|')
                .filter_map(|x| x.trim().parse().ok())
                .collect();
            if parts.len() == 2 {
                rules.push((parts[0], parts[1]));
            }
        } else if line.contains(',') {
            let update: Vec<i32> = line
                .split(',')
                .filter_map(|x| x.trim().parse().ok())
                .collect();
            updates.push(update);
        }
    }

    Ok((rules, updates))
}

fn main() -> io::Result<()> {
    let (rules, updates) = read_rules_and_updates("input.txt")?;

    let total = find_correctly_ordered_updates(&rules, &updates);
    println!("The sum of middle page numbers is: {:?}", total);

    Ok(())
}

fn find_correctly_ordered_updates(rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut total_middle_sum = 0;
    let mut updated_total_middle_sum = 0;

    for update in updates {
        if is_correctly_ordered(update, rules) {
            let middle_index = update.len() / 2;
            total_middle_sum += update[middle_index];
        } else {
            let corrected_update = correct_order(update, rules);
            let middle_index = corrected_update.len() / 2;
            updated_total_middle_sum += corrected_update[middle_index];
        }
    }

    (total_middle_sum, updated_total_middle_sum)
}

fn is_correctly_ordered(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let position_map: HashMap<i32, usize> = update
        .iter()
        .enumerate()
        .map(|(index, &page)| (page, index))
        .collect();

    for &(before, after) in rules {
        if let (Some(&pos_before), Some(&pos_after)) = (position_map.get(&before), position_map.get(&after)) {
            if pos_before >= pos_after {
                return false;
            }
        }
    }

    true 
}

fn correct_order(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut sorted_update = update.clone();
    let rule_set: HashSet<(i32, i32)> = rules.iter().cloned().collect();

    sorted_update.sort_by(|&a, &b| {
        if rule_set.contains(&(a, b)) {
            std::cmp::Ordering::Less
        } else if rule_set.contains(&(b, a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    sorted_update
}