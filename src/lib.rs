use std::{error::Error, fs, process, str::Chars};

pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let rucksacks: Vec<Rucksack> = content
        .trim()
        .lines()
        .map(|line| {
            let line_length = line.len();
            let compartment_1 = &line[..line_length / 2];
            let compartment_2 = &line[(line_length / 2)..];
            let repeated_items = find_repeated_items(line);

            Rucksack {
                compartment_1,
                compartment_2,
                repeated_items,
            }
        })
        .collect();

    let sum: u32 = rucksacks
        .iter()
        .map(|r| {
            let mut encountered = String::new();
            r.repeated_items
                .iter()
                .filter(|c| {
                    if encountered.contains(**c) {
                        false
                    } else {
                        encountered.push(**c);
                        true
                    }
                })
                .map(|c| {
                    c.prioritese()
                })
                .sum::<u32>()
        }).sum();

    println!("Value of repeated items summed up is {sum}");

    Ok(())
}

pub fn find_repeated_items(line: &str) -> Vec<char> {
    let first_half = &line[..line.len() / 2];
    let second_half = &line[line.len() / 2..];

    first_half
        .chars()
        .filter(|c| second_half.contains(*c))
        .collect()
}

struct Rucksack<'a> {
    compartment_1: &'a str,
    compartment_2: &'a str,
    repeated_items: Vec<char>,
}

pub trait Prioritise {
    fn prioritese(self: &Self) -> u32;
}

impl Prioritise for char {
    fn prioritese(self: &Self) -> u32 {
        match *self {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 1 + 26,
            'B' => 2 + 26,
            'C' => 3 + 26,
            'D' => 4 + 26,
            'E' => 5 + 26,
            'F' => 6 + 26,
            'G' => 7 + 26,
            'H' => 8 + 26,
            'I' => 9 + 26,
            'J' => 10 + 26,
            'K' => 11 + 26,
            'L' => 12 + 26,
            'M' => 13 + 26,
            'N' => 14 + 26,
            'O' => 15 + 26,
            'P' => 16 + 26,
            'Q' => 17 + 26,
            'R' => 18 + 26,
            'S' => 19 + 26,
            'T' => 20 + 26,
            'U' => 21 + 26,
            'V' => 22 + 26,
            'W' => 23 + 26,
            'X' => 24 + 26,
            'Y' => 25 + 26,
            'Z' => 26 + 26,
            _ => {
                eprint!("Error: Invalid char to prioritise.");
                eprint!("Must be a char in the range of a-z A-Z");
                process::exit(1)
            }
        }
    }
}
