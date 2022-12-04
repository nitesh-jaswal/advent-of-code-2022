use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn lazy_file_reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn _get_item_priority(item: &char) -> Option<u32> {
    let item_value = *item as u32;
    if item.is_ascii_uppercase() {
        Some(item_value - 38)
    }
    else if item.is_ascii_lowercase() {
        Some(item_value - 96)
    }
    else {
        None
    }
}

fn _get_group_badge_item_priority(group_items: &Vec<String>) -> u32 {
    let mut badge_item: HashSet<char> = group_items[0].chars().collect();
    for items in group_items[1..].iter() {
        let item_set: HashSet<char> = items.chars().collect();
        badge_item = badge_item.intersection(&item_set).cloned().collect();
    }
    if badge_item.len() > 1 {
        panic!("Invalid input! Multiple badges found for a group!")
    }

    if let Some(badge_item_priority) = _get_item_priority(badge_item.iter().next().unwrap()) {
        return badge_item_priority
    }
    else {
        panic!("Invalid item encountered! Please ask the elves to recheck the item list");
    }
}

pub fn run() -> u32 {
    let input_path = "../../input.txt";
    let mut total_badge_priority: u32 = 0;
    let mut line_count: u32 = 0;
    let mut group_lines: Vec<String> = Vec::new();
    if let Ok(lines) = lazy_file_reader(input_path) {
        for line in lines {
            if let Ok(rucksack_items) = line {
                group_lines.push(rucksack_items);
                line_count += 1;
                if line_count == 3 {
                    let badge_item_priority = _get_group_badge_item_priority(&group_lines);
                    total_badge_priority += badge_item_priority;
                    line_count = 0;
                    group_lines.clear();
                }
            }
        }
    }
    total_badge_priority
}