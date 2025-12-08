use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;
use std::{env, fs};

#[derive(Eq, PartialEq)]
struct RelativeVector {
    i: usize,
    j: usize,
    dist_sq: i64,
}

impl Ord for RelativeVector {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist_sq.cmp(&self.dist_sq)
    }
}

impl PartialOrd for RelativeVector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_and_parse_input() -> (Vec<Vec<i64>>, usize) {
    let args = env::args().collect::<Vec<_>>();

    let filepath = &args[1];
    let pairs = args[2].parse::<usize>().unwrap();

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    let junctions = input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|pc| pc.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (junctions, pairs)
}

fn solve(junctions: &Vec<Vec<i64>>, pairs: usize) -> (i64, i64) {
    let mut distances = BinaryHeap::<RelativeVector>::new();

    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let dist_sq = (0..3)
                .map(|d| {
                    let delta = junctions[i][d] - junctions[j][d];
                    delta * delta
                })
                .sum::<i64>();

            let rel_vec = RelativeVector { i, j, dist_sq };
            distances.push(rel_vec)
        }
    }

    let mut contained_in = (0..junctions.len()).collect::<Vec<_>>();
    let mut subsets: Vec<Vec<usize>> = (0..junctions.len()).map(|i| vec![i]).collect::<Vec<_>>();

    let mut part_one: Option<i64> = None;

    let mut edge_count = 0;
    while let Some(rel_vec) = distances.pop() {
        if edge_count == pairs {
            let unique_subsets = contained_in.iter().cloned().collect::<HashSet<_>>();
            let mut counts = unique_subsets
                .iter()
                .map(|set_idx| subsets[*set_idx].len() as i64)
                .collect::<Vec<_>>();
            counts.sort();
            counts.reverse();
            part_one = Some(counts.iter().take(3).product());
        }

        edge_count += 1;

        let i = rel_vec.i;
        let j = rel_vec.j;

        if contained_in[i] == contained_in[j] {
            continue;
        }

        let j_set_idx = contained_in[j];
        let j_set = subsets[j_set_idx].clone();

        let i_set_idx = contained_in[i];

        for &k in j_set.iter() {
            contained_in[k] = i_set_idx;
        }

        let i_set = subsets.get_mut(i_set_idx).unwrap();

        i_set.extend(j_set);

        if i_set.len() == junctions.len() {
            let part_two = junctions[i][0] * junctions[j][0];
            return (part_one.unwrap(), part_two);
        }
    }

    panic!()
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 8: Playground");

    let now = Instant::now();

    let (junctions, pairs) = read_and_parse_input();

    let (part_one, part_two) = solve(&junctions, pairs);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
