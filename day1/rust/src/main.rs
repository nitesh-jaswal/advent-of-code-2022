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
    let mut elves: Vec<ElfCalories> = vec![];
    let mut max_calorie_elf: u32 = 0;
    if let Ok(lines) = lazy_file_reader(input_path) {
        let mut elf_calories: Vec<u32> = vec![];
        for line in lines {
            if let Ok(calorie) = line {
                if calorie == "" {
                    let elf = ElfCalories::new(&elf_calories);
                    if elf.total_calories > max_calorie_elf {
                        max_calorie_elf = elf.total_calories
                    }
                    elves.push(elf);
                    let mut elf_calories: Vec<u32> = vec![];
                    continue;
                }
                elf_calories.push(calorie.parse::<u32>().unwrap());
            }
        }
    }
    print!("The elf with maximum calories has {} calories", max_calorie_elf);
}
