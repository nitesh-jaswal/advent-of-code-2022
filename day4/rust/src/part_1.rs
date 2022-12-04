use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct SectionAssignment{
    start: i32,
    end: i32
}

impl SectionAssignment {
    fn new(pair: &Vec<&str>) -> Self {
        Self {
            start: pair[0].parse::<i32>().unwrap(),
            end: pair[1].parse::<i32>().unwrap()
        }
    }
}

fn lazy_file_reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_pair_overlap(assignment_pair: &Vec<&str>) -> bool {
    let first_section = SectionAssignment::new(&assignment_pair[0].split("-").collect::<Vec<&str>>());
    let second_section = SectionAssignment::new(&assignment_pair[1].split("-").collect::<Vec<&str>>());
    let val_1 = second_section.start - first_section.start;
    let val_2 = second_section.end - first_section.end;
    if val_1 >= 0 && val_2 <= 0 {
        true
    }
    else if val_1 <= 0 && val_2 >= 0 {
        true
    }
    else {
        false
    }
}

pub fn run() -> u32 {
    let input_path = "../../input.txt";
    let mut num_full_overlaps: u32 = 0;
    if let Ok(lines) = lazy_file_reader(input_path) {
        for line in lines {
            if let Ok(assignment_pair) = line {
                let assignment_pair = assignment_pair.split(",").collect::<Vec<&str>>();
                if is_pair_overlap(&assignment_pair) {
                    num_full_overlaps += 1;
                }
            }
        }
    }
    num_full_overlaps
}