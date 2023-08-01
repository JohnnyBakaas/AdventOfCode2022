/*
How to run:
"cd" inn i mappe
"rustc {prog}.rs"
"./{prog}.exe"
*/

use std::fs;

fn main() {
    let lines = get_list();
    let mut most_largest: u32 = 0;
    let mut temp: u32 = 0;

    for line in &lines {
        if line.len() > 1 {
            let numb: u32 = clac_total_part(line.to_string());
            println!("part {}", line);
            print_type_of(&numb);
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

fn get_list() -> Vec<String> {
    let file_path = "C:/Users/johnn/OneDrive/Dokumenter/GitHub/cv-generic-react/AdventOfCode2022/Rust/Day1/adventofcode.com_2022_day_1_input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    return lines;
}

fn clac_total_part(inp: String) -> u32 {
    let num: u32 = inp.parse().unwrap();
    return num;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
