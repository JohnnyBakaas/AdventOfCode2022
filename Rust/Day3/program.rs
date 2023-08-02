use std::fs;

fn main() {
    part_two();
}

fn part_one() {
    let lines = get_list();
    let mut total_count = 0;

    for line in lines {
        let prio = char_to_prio(part_string(line));
        println!("{}", prio);
        total_count += prio;
    }

    println!("total {}", total_count);
}

fn part_two() {
    let lines = get_list();
    let mut total_count = 0;

    for i in (0..lines.len()).step_by(3) {
        let mut char_codes: [u32; 53] = [0; 53];

        for c in lines[i].chars() {
            char_codes[char_to_prio(c) as usize] = 1;
        }

        for c in lines[i + 1].chars() {
            if char_codes[char_to_prio(c) as usize] == 1 {
                char_codes[char_to_prio(c) as usize] = 2;
            }
        }

        for c in lines[i + 2].chars() {
            if char_codes[char_to_prio(c) as usize] == 2 {
                char_codes[char_to_prio(c) as usize] = 3;
            }
        }

        for (i, code) in char_codes.iter().enumerate() {
            if *code == 3 {
                total_count += i;
            }
        }
    }
    println!("total {}", total_count);
}

fn char_to_prio(c: char) -> u32 {
    let b = c as u32;
    if b >= 97 {
        return b - 96;
    }
    return b - 38;
}

fn part_string(inp: String) -> char {
    let xstring = inp.chars();
    let half_length = (inp.len() + 1) / 2;
    for c in xstring.clone().take(half_length) {
        for c2 in xstring.clone().skip(half_length) {
            if c == c2 {
                return c;
            }
        }
    }
    println!("HEIIIII Her er det en FEIL");
    return 'a';
}

fn get_list() -> Vec<String> {
    let file_path = "C:/Users/johnn/OneDrive/Dokumenter/GitHub/cv-generic-react/AdventOfCode2022/Rust/Day3/adventofcode.com_2022_day_3_input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    return lines;
}
