use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::{env, fs};

const DEFAULT_FILEPATH: &str = "./input/input.txt";

struct Manifold {
    start: u64,
    splitters: Vec<HashSet<u64>>,
}

fn read_and_parse_input() -> Manifold {
    let args = env::args().collect::<Vec<_>>();

    let filepath = if args.len() > 1 {
        &args[1]
    } else {
        DEFAULT_FILEPATH
    };

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    let chars = input
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                Some(line.chars().collect::<Vec<_>>())
            }
        })
        .collect::<Vec<_>>();

    let start = chars[0]
        .iter()
        .enumerate()
        .find(|(_, c)| **c == 'S')
        .unwrap()
        .0 as u64;
    let splitters = chars[1..]
        .iter()
        .filter_map(|row| {
            let s = row
                .iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == '^' { Some(i as u64) } else { None })
                .collect::<HashSet<_>>();
            if s.is_empty() { None } else { Some(s) }
        })
        .collect::<Vec<_>>();

    Manifold { start, splitters }
}

impl Manifold {
    fn solve(&self) -> (u64, u64) {
        let mut part_one = 0;
        let mut timelines = HashMap::new();
        timelines.insert(self.start, 1u64);

        for splitters_at_depth in self.splitters.iter() {
            let mut new_timelines = HashMap::new();

            for (pos, timeline_count) in timelines.iter() {
                if splitters_at_depth.contains(&pos) {
                    part_one += 1;
                    new_timelines.insert(
                        pos - 1,
                        timeline_count + new_timelines.get(&(pos - 1)).unwrap_or(&0),
                    );
                    new_timelines.insert(
                        pos + 1,
                        timeline_count + new_timelines.get(&(pos + 1)).unwrap_or(&0),
                    );
                } else {
                    new_timelines.insert(
                        *pos,
                        timeline_count + new_timelines.get(&(pos)).unwrap_or(&0),
                    );
                }
            }
            timelines = new_timelines;
        }

        let total_timelines = timelines.values().sum::<u64>();
        (part_one, total_timelines)
    }
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 7: Laboratories");

    let now = Instant::now();

    let manifold = read_and_parse_input();

    let (part_one, part_two) = manifold.solve();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
