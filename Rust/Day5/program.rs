use std::fs;
use std::str::FromStr;
fn main() {
    part_one()
}

fn part_one() {
    let lines = get_list();

    let mut stacks: [Vec<String>; 9] = Default::default();

    for line in lines.iter().take(8).rev() {
        println!("{}", line);
        if let Some(ch) = line.chars().nth(1) {
            stacks[0].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(5) {
            stacks[1].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(9) {
            stacks[2].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(13) {
            stacks[3].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(17) {
            stacks[4].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(21) {
            stacks[5].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(25) {
            stacks[6].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(29) {
            stacks[7].push(ch.to_string());
        }
        if let Some(ch) = line.chars().nth(33) {
            stacks[8].push(ch.to_string());
        }
    }

    for e in &stacks[0] {
        println!("e {}", e);
    }

    for line in lines.iter().skip(10) {
        let moves = parse_input(line.to_string());

        println!("move {}", moves[0]);
        println!("from {}", moves[1]);
        println!("to   {}", moves[2]);

        for _i in 0..moves[0] {
            let from_index = (moves[1] - 1) as usize;
            let to_index = (moves[2] - 1) as usize;

            for _i in 0..moves[0] {
                if let Some(e) = stacks[from_index].pop() {
                    stacks[to_index].push(e);
                }
            }
        }
    }

    for stack in &stacks {
        let mut kake = String::new();
        for f in stack {
            kake.push_str(f);
        }
        println!("{}", kake);
    }
}

fn get_list() -> Vec<String> {
    let file_path = "C:/Users/johnn/OneDrive/Dokumenter/GitHub/cv-generic-react/AdventOfCode2022/Rust/Day5/adventofcode.com_2022_day_5_input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    return lines;
}

fn parse_input(input: String) -> [i32; 3] {
    let parts: Vec<&str> = input.split(' ').collect();

    let mut result: [i32; 3] = [
        FromStr::from_str(parts[1]).unwrap(),
        FromStr::from_str(parts[3]).unwrap(),
        FromStr::from_str(parts[5]).unwrap(),
    ];

    return result;
}
