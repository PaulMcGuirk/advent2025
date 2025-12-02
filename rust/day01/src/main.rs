use std::time::Instant;
use std::{env, fs};

const DEFAULT_FILEPATH: &str = "./input/input.txt";

fn read_and_parse_input() -> Vec<(i32, u32)> {
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
            let dir = match line.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!(),
            };
            let dist = line[1..].parse::<u32>().unwrap();
            (dir, dist)
        })
        .collect()
}

fn solve(turns: &Vec<(i32, u32)>, start: u32, dial_size: u32) -> (u32, u32) {
    let mut pos = start as i32;

    let mut landed_count = 0;
    let mut passed_count = 0;

    for &(dir, dist) in turns.iter() {
        let prev_pos = pos;
        let total_turns = dist / dial_size;
        let rest = dist % dial_size;

        passed_count += total_turns;

        pos += dir * rest as i32;
        if (prev_pos != 0 && pos < 0) || pos > dial_size as i32 {
            passed_count += 1;
        }
        pos = pos.rem_euclid(dial_size as i32);

        if pos == 0 {
            landed_count += 1;
        }
    }

    (landed_count, landed_count + passed_count)
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 1: Secret Entrance");

    let now = Instant::now();

    let turns = read_and_parse_input();

    let start = 50;
    let size = 100;

    let (part_one, part_two) = solve(&turns, start, size);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
