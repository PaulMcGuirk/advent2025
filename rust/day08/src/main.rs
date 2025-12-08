use std::collections::HashSet;
use std::time::Instant;
use std::{env, fs};

struct Playground {
    junctions: Vec<Vec<i64>>,
    distances: Vec<(usize, usize, i64)>,
}

fn read_and_parse_input() -> (Playground, usize) {
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

    let distances = {
        // can convert to a binary heap if needed
        let mut distances = (0..junctions.len())
            .flat_map(|i| {
                ((i + 1)..junctions.len())
                    .map(|j| {
                        let dist_sq = (0..3)
                            .map(|d| {
                                let delta = junctions[i][d] - junctions[j][d];
                                delta * delta
                            })
                            .sum::<i64>();
                        (i, j, dist_sq)
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect::<Vec<_>>();
        distances.sort_by_key(|(_, _, dist_sq)| *dist_sq);
        distances
    };

    let playground = Playground {
        junctions,
        distances,
    };

    (playground, pairs)
}

impl Playground {
    fn solve_part_one(&self, pairs: usize) -> u64 {
        let edges = {
            let mut edges = vec![vec![]; self.junctions.len()];
            for &(i, j, _) in self.distances.iter().take(pairs) {
                edges[i].push(j);
                edges[j].push(i);
            }
            edges
        };

        let mut seen = HashSet::new();
        let mut pieces = vec![];

        for &(i, _, _) in self.distances.iter().take(pairs) {
            if seen.contains(&i) {
                continue;
            }

            let mut to_visit = vec![i];
            let mut sub = vec![];

            while let Some(next) = to_visit.pop() {
                if seen.contains(&next) {
                    continue;
                }
                seen.insert(next);
                sub.push(next);

                to_visit.extend(edges[next].clone());
            }

            pieces.push(sub);
        }

        pieces.sort_by_key(|sub| self.junctions.len() - sub.len());
        pieces.iter().take(3).map(|sub| sub.len() as u64).product()
    }

    fn solve_part_two(&self) -> i64 {
        let mut contained_in = (0..self.junctions.len()).collect::<Vec<_>>();
        let mut subsets: Vec<HashSet<usize>> = (0..self.junctions.len())
            .map(|i| HashSet::from_iter([i].into_iter()))
            .collect::<Vec<_>>();

        for &(i, j, _) in self.distances.iter() {
            if contained_in[i] == contained_in[j] {
                continue; // already in same set, can continue
            }

            // merge j into i
            let j_set_idx = contained_in[j];
            let j_set = subsets[j_set_idx].clone();

            let i_set_idx = contained_in[i];

            for &k in j_set.iter() {
                contained_in[k] = i_set_idx;
            }

            let i_set = subsets.get_mut(i_set_idx).unwrap();

            i_set.extend(j_set);

            if i_set.len() == self.junctions.len() {
                return self.junctions[i][0] * self.junctions[j][0];
            }
        }

        panic!()
    }
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 8: Playground");

    let now = Instant::now();

    let (playground, pairs) = read_and_parse_input();

    let part_one = playground.solve_part_one(pairs);
    let part_two = playground.solve_part_two();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
