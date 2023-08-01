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
    let mut most_largest: u32 = 0;
    let mut temp: u32 = 0;

    for line in &lines {
        if line.len() > 1 {
            let numb: u32 = string_to_u32(line.to_string());
            temp += numb;
        } else {
            if most_largest < temp {
                most_largest = temp
            }

            temp = 0
        }
    }

    println!("most_largest {}", most_largest);
}

fn part_two() {
    let lines = get_list();
    let mut most_largest: u32 = 0;
    let mut second_most_largest: u32 = 0;
    let mut third_most_largest: u32 = 0;
    let mut temp: u32 = 0;

    for line in &lines {
        if line.len() > 1 {
            let numb: u32 = string_to_u32(line.to_string());
            temp += numb;
        } else {
            if most_largest < temp {
                third_most_largest = second_most_largest;
                second_most_largest = most_largest;
                most_largest = temp;
            } else if second_most_largest < temp {
                third_most_largest = second_most_largest;
                second_most_largest = temp;
            } else if third_most_largest < temp {
                third_most_largest = temp;
            }

            temp = 0
        }
    }

    println!(
        "most_total_largest {}",
        most_largest + second_most_largest + third_most_largest
    );
}

fn get_list() -> Vec<String> {
    let file_path = "C:/Users/johnn/OneDrive/Dokumenter/GitHub/cv-generic-react/AdventOfCode2022/Rust/Day1/adventofcode.com_2022_day_1_input.txt";
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
