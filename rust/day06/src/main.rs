use std::time::Instant;
use std::{env, fs};

const DEFAULT_FILEPATH: &str = "./input/input.txt";

enum Op {
    Add,
    Mul,
}

fn read_and_parse_input() -> Vec<(Op, Vec<Vec<u64>>)> {
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

    let mut problems = vec![];
    let num_rows = chars.len();
    let num_cols = chars[0].len();

    let mut c = 0;
    while c < num_cols {
        // the operator positions define the column
        let c_init = c;
        let op = match chars[num_rows - 1][c] {
            '+' => Op::Add,
            '*' => Op::Mul,
            ch => panic!("Expected op got {ch}"),
        };

        c += 1;
        while c < num_cols && chars[num_rows - 1][c] == ' ' {
            c += 1;
        }

        let normal_nums = (0..(num_rows - 1))
            .map(|r| {
                chars[r]
                    .iter()
                    .skip(c_init)
                    .take(c - c_init)
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let cephalopod_nums = (c_init..c)
            .rev()
            .filter_map(|cc| {
                let s = (0..(num_rows - 1))
                    .map(|r| chars[r][cc])
                    .collect::<String>();
                if s.trim().is_empty() {
                    None
                } else {
                    Some(s.trim().parse::<u64>().unwrap())
                }
            })
            .collect::<Vec<u64>>();
        problems.push((op, vec![normal_nums, cephalopod_nums]))
    }

    problems
}

fn solve(problems: &Vec<(Op, Vec<Vec<u64>>)>) -> (u64, u64) {
    let res = problems
        .iter()
        .fold(vec![0u64, 0u64], |res_vec, (op, nums_vec)| {
            res_vec
                .iter()
                .enumerate()
                .map(|(i, res)| {
                    res + match op {
                        Op::Add => nums_vec[i].iter().sum::<u64>(),
                        Op::Mul => nums_vec[i].iter().product(),
                    }
                })
                .collect::<Vec<u64>>()
        });
    (res[0], res[1])
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 6: Trash Compactor");

    let now = Instant::now();

    let problems = read_and_parse_input();

    let (part_one, part_two) = solve(&problems);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
