use std::fs;
fn main() {
    println!("");
    part_one();
    println!("");
    part_two();
    println!("");
}

fn part_one() {
    let lines = get_list();
    let mut count = 0;

    for line in lines {
        let parsed_line = parse_input(line);

        if (parsed_line[0] <= parsed_line[2] && parsed_line[1] >= parsed_line[3])
            || (parsed_line[0] >= parsed_line[2] && parsed_line[1] <= parsed_line[3])
        {
            count += 1;
        }
    }

    println!("count {}", count)
}

fn part_two() {
    let lines = get_list();
    let mut count = 0;

    for line in lines {
        let parsed_line = parse_input(line);

        // Ikke se på dette som min kode kunskap, jeg er veldig trøtt
        let mut counter: [u32; 100] = [0; 100];

        for i in parsed_line[0]..parsed_line[1] + 1 {
            counter[i as usize] += 1
        }

        for i in parsed_line[2]..parsed_line[3] + 1 {
            counter[i as usize] += 1;
            if counter[i as usize] == 2 {
                count += 1;
                break;
            }
        }
    }
    println!("count {}", count)
}

fn get_list() -> Vec<String> {
    let file_path = "C:/Users/johnn/OneDrive/Dokumenter/GitHub/cv-generic-react/AdventOfCode2022/Rust/Day4/adventofcode.com_2022_day_4_input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    return lines;
}

fn parse_input(input: String) -> [u32; 4] {
    let mut result: [u32; 4] = [0; 4];

    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() == 2 {
        let left_numbers: Vec<u32> = parts[0]
            .split('-')
            .map(|num| num.trim().parse().unwrap_or_default())
            .collect();

        let right_numbers: Vec<u32> = parts[1]
            .split('-')
            .map(|num| num.trim().parse().unwrap_or_default())
            .collect();

        if left_numbers.len() == 2 && right_numbers.len() == 2 {
            result[0] = left_numbers[0];
            result[1] = left_numbers[1];
            result[2] = right_numbers[0];
            result[3] = right_numbers[1];
        }
    }

    return result;
}
