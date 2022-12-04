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

fn _get_errored_item_priority(rucksack_items: String) -> Option<u32> {
    let total_items = rucksack_items.len();
    let midpoint = total_items/2;
    let first_compartment_items = rucksack_items[0..midpoint].chars().collect::<HashSet<char>>();
    for second_compartment_item in rucksack_items[midpoint..total_items].chars() {
        if first_compartment_items.contains(&second_compartment_item) {
            if let Some(priority) = _get_item_priority(&second_compartment_item) {
                return  Some(priority)
            }
            else {
                panic!("Invalid item encountered! Please ask the elves to recheck the item list");
            }
        }
    }
    None
}

pub fn run() -> u32 {
    let input_path = "../../input.txt";
    let mut total_error_priority: u32 = 0;
    if let Ok(lines) = lazy_file_reader(input_path) {
        for line in lines {
            if let Ok(rucksack_items) = line {
                let errored_item = _get_errored_item_priority(rucksack_items);
                if let Some(errored_item) = errored_item {
                    total_error_priority += errored_item
                }
            }
        }
    }
    total_error_priority
}