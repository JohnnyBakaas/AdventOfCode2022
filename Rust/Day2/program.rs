/*
How to run:
"cd" inn i mappe
"rustc {prog}.rs"
"./{prog}.exe"
*/

use std::fs;

fn main() {
    part_two()
}

fn part_one() {
    let lines = get_list();
    let mut total_count = 0;

    for line in &lines {
        let other = other_u32(line.to_string());
        let my = my_u32(line.to_string());

        if my == other {
            total_count += 3;
        } else if my - 1 == other % 3 {
            total_count += 6;
        }

        total_count += my;
    }

    println!("{}", total_count);
}

fn part_two() {
    let lines = get_list();
    let mut total_count = 0;

    for line in &lines {
        let other = other_u32(line.to_string());
        let my = my_u32(line.to_string());

        if my == 1 {
            if other == 1 {
                total_count += 3
            } else {
                total_count += other - 1
            }
        }

        if my == 2 {
            total_count += 3;
            total_count += other;
        }
        if my == 3 {
            total_count += 6;
            if other == 3 {
                total_count += 1
            } else {
                total_count += other + 1
            }
        }
    }

    println!("{}", total_count);
}

fn other_u32(inp: String) -> u32 {
    let b = inp.as_bytes();
    return (b[0] - 64).into();
}

fn my_u32(inp: String) -> u32 {
    let b = inp.as_bytes();
    return (b[2] - 87).into();
}

fn get_list() -> Vec<String> {
    let file_path = "C:/Users/johnn/OneDrive/Dokumenter/GitHub/cv-generic-react/AdventOfCode2022/Rust/Day2/adventofcode.com_2022_day_2_input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    return lines;
}

fn string_to_u32(inp: String) -> u32 {
    let num: u32 = inp.parse().unwrap();
    return num;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
