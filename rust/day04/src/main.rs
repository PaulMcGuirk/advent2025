use std::time::Instant;
use std::{env, fs};

const DEFAULT_FILEPATH: &str = "./input/input.txt";

fn read_and_parse_input() -> Vec<Vec<bool>> {
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
        .map(|line| line.trim().chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn remove(grid: &Vec<Vec<bool>>) -> (usize, Vec<Vec<bool>>) {
    let mut updated_grid = vec![];
    let mut removed = 0;

    for r in 0..grid.len() {
        let mut row = vec![];
        for c in 0..grid[0].len() {
            if !grid[r][c] {
                row.push(false);
            } else {
                let num_neighbors = {
                    let mut num_neighbors = 0;
                    for d_r in (-1)..=1 {
                        let n_r = r as i32 + d_r;
                        if n_r < 0 {
                            continue;
                        }
                        let n_r = n_r as usize;
                        if n_r >= grid.len() {
                            continue;
                        }
                        for d_c in (-1)..=1 {
                            let n_c = c as i32 + d_c;
                            if n_c < 0 {
                                continue;
                            }
                            let n_c = n_c as usize;
                            if n_c >= grid[0].len() {
                                continue;
                            }
                            if (n_r, n_c) != (r, c) && grid[n_r][n_c] {
                                num_neighbors += 1;
                            }
                        }
                    }
                    num_neighbors
                };

                if num_neighbors < 4 {
                    row.push(false);
                    removed += 1;
                } else {
                    row.push(true);
                }
            }
        }
        updated_grid.push(row);
    }

    (removed, updated_grid)
}

fn solve(grid: &Vec<Vec<bool>>) -> (usize, usize) {
    let (mut part_two, mut grid) = remove(&grid);
    let part_one = part_two;

    loop {
        let (r, g) = remove(&grid);
        if r == 0 {
            break;
        }
        part_two += r;
        grid = g;
    }

    (part_one, part_two)
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 4: Printing Department");

    let now = Instant::now();

    let grid = read_and_parse_input();

    let (part_one, part_two) = solve(&grid);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
