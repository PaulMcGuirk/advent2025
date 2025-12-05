use std::time::Instant;
use std::{env, fs};

const DEFAULT_FILEPATH: &str = "./input/input.txt";

fn read_and_parse_input() -> (Vec<(u64, u64)>, Vec<u64>) {
    let args = env::args().collect::<Vec<_>>();

    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_FILEPATH
    };

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    let mut chunks = input.trim().split("\n\n");

    let ranges = chunks
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut pcs = line.split('-');
            (
                pcs.next().unwrap().parse::<u64>().unwrap(),
                pcs.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let ids = chunks
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (ranges, ids)
}

fn solve_part_one(ranges: &Vec<(u64, u64)>, ids: &Vec<u64>) -> usize {
    ids.iter()
        .filter(|&&id| {
            ranges
                .iter()
                .any(|&(lower, upper)| id >= lower && id <= upper)
        })
        .count()
}

fn solve_part_two(ranges: &Vec<(u64, u64)>) -> u64 {
    let sorted_ranges = {
        let mut sorted_ranges = ranges.clone();
        sorted_ranges.sort_by_key(|rg| rg.0);
        sorted_ranges
    };

    let merged_ranges = {
        let mut merged_ranges = vec![];
        let mut i = 0;
        while i < sorted_ranges.len() {
            let mut next = sorted_ranges[i].clone();
            i += 1;
            while i < sorted_ranges.len() && sorted_ranges[i].0 <= next.1 {
                next.1 = next.1.max(sorted_ranges[i].1);
                i += 1;
            }
            merged_ranges.push(next);
        }
        merged_ranges
    };

    merged_ranges
        .iter()
        .map(|range| range.1 - range.0 + 1)
        .sum()
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 4: Cafeteria");

    let now = Instant::now();

    let (ranges, ids) = read_and_parse_input();

    let part_one = solve_part_one(&ranges, &ids);
    let part_two = solve_part_two(&ranges);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
