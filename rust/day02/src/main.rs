use std::time::Instant;
use std::{env, fs};

const DEFAULT_FILEPATH: &str = "./input/input.txt";

fn read_and_parse_input() -> Vec<(u64, u64)> {
    let args = env::args().collect::<Vec<_>>();

    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_FILEPATH
    };

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    input
        .trim()
        .split(',')
        .map(|range| {
            let mut bounds = range.split('-');
            (
                bounds.next().unwrap().parse().unwrap(),
                bounds.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn is_invalid_helper(s: &str, repeats: usize) -> bool {
    // regex crate doesn't support backreference, so here we are
    if s.len() % repeats > 0 {
        return false;
    }

    let seg_length = s.len() / repeats;

    let to_match = &s[0..seg_length];

    for i in 1..repeats {
        if &s[(seg_length * i)..(seg_length * (i + 1))] != to_match {
            return false;
        }
    }

    true
}

fn is_invalid_part_one(id: u64) -> bool {
    let s = id.to_string();

    is_invalid_helper(&s, 2)
}

fn is_invalid_part_two(id: u64) -> bool {
    let s = id.to_string();

    (2..=s.len()).any(|r| is_invalid_helper(&s, r))
}

fn solve_part_one(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges
        .iter()
        .map(|range| {
            (range.0..=range.1)
                .filter(|&id| is_invalid_part_one(id))
                .sum::<u64>()
        })
        .sum()
}

fn solve_part_two(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges
        .iter()
        .map(|range| {
            (range.0..=range.1)
                .filter(|&id| is_invalid_part_two(id))
                .sum::<u64>()
        })
        .sum()
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 2: Gift Shop");

    let now = Instant::now();

    let ranges = read_and_parse_input();

    let part_one = solve_part_one(&ranges);
    let part_two = solve_part_two(&ranges);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
