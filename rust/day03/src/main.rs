use std::time::Instant;
use std::{env, fs};

const DEFAULT_FILEPATH: &str = "./input/input.txt";

fn read_and_parse_input() -> Vec<Vec<u32>> {
    let args = env::args().collect::<Vec<_>>();

    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_FILEPATH
    };

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn solve(banks: &Vec<Vec<u32>>, group_count: usize) -> u64 {
    banks
        .iter()
        .map(|bank| largest_group(bank, group_count))
        .sum()
}

fn largest_group(bank: &Vec<u32>, group_count: usize) -> u64 {
    let mut res = 0;
    let mut start_idx = 0;

    for i in 0..group_count {
        let (idx, elem) = bank
            .iter()
            .enumerate()
            .take(bank.len() - (group_count - i - 1))
            .skip(start_idx)
            .rev() // in the case of a tie, max_by_key returns the last, while we need the first
            .max_by_key(|pair| pair.1)
            .unwrap();
        start_idx = idx + 1;
        res = 10 * res + (*elem as u64);
    }

    res
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 3: Lobby");

    let now = Instant::now();

    let banks = read_and_parse_input();

    let part_one = solve(&banks, 2);
    let part_two = solve(&banks, 12);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
