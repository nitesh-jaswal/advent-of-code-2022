use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct ElfCalories {
    fruit_list: Vec<u32>,
    total_calories: u32
}

impl ElfCalories {
    fn new(input_vec: &Vec<u32>) -> Self {
        Self {
            fruit_list: input_vec.to_vec(),
            total_calories: input_vec.iter().sum::<u32>()
        }
    }
}
// Reference: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn lazy_file_reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let input_path = "../input.txt";
    let mut elves_max_calories: Vec<u32> = Vec::new();
    if let Ok(lines) = lazy_file_reader(input_path) {
        let mut elf_calories: Vec<u32> = Vec::new();
        for line in lines {
            if let Ok(calorie) = line {
                if calorie == "" {
                    let elf = ElfCalories::new(&elf_calories);
                    elves_max_calories.push(elf.total_calories);
                    elf_calories.clear();
                    continue;
                }
                elf_calories.push(calorie.parse::<u32>().unwrap());
            }
        }
    }
    elves_max_calories.sort();
    elves_max_calories.reverse();
    println!("The elf with maximum calories has {} calories", elves_max_calories[0]);
    println!("The total calories by top 3 elves is {}", elves_max_calories[0..=2].iter().sum::<u32>());
}
