use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use std::{env, fs, u64};

struct Reactor {
    out_edges: Vec<Vec<usize>>,
    in_edges: Vec<Vec<usize>>,
    ids: HashMap<String, usize>,
}

fn read_and_parse_input() -> Reactor {
    let args = env::args().collect::<Vec<_>>();

    let filepath = &args[1];

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    let mut all_ids = HashMap::<String, usize>::new();
    let mut out_edges = vec![];
    let mut in_edges = vec![];

    for line in input.trim().lines() {
        let nodes = line
            .trim()
            .replace(":", "")
            .split_ascii_whitespace()
            .map(|pc| String::from(pc))
            .collect::<Vec<_>>();
        let mut ids = vec![];
        for node in nodes.iter() {
            let id = {
                if let Some(id) = all_ids.get(node) {
                    *id
                } else {
                    all_ids.insert(node.clone(), out_edges.len());
                    out_edges.push(vec![]);
                    in_edges.push(vec![]);
                    all_ids[node]
                }
            };
            ids.push(id);
        }
        out_edges[ids[0]] = ids[1..].iter().cloned().collect::<Vec<_>>();
        for &id in ids.iter().skip(1) {
            in_edges[id].push(ids[0]);
        }
    }

    Reactor {
        in_edges,
        out_edges,
        ids: all_ids,
    }
}

impl Reactor {
    fn solve(&self, source: &str, target: &str, reqs: Vec<&str>) -> u64 {
        let source = self.ids[source];
        let target = self.ids[target];
        let reqs = reqs.iter().map(|&r| self.ids[r]).collect::<Vec<_>>();
        let has_reqs = reqs.len() > 0;
        let path_count_length = if has_reqs { reqs.len() + 2 } else { 1 };

        // when there is a list of r requirements, we'll keep a vector
        // where the first entry is the total number of paths to that point,
        // followed r entries representing the number of paths to that point
        // containing the corresponding column, and then a final entry representing
        // the total number of paths to that point containing all requirements
        //
        // when there are no requirements, the vector will have a single entry
        // for the total number of paths to that point
        let mut path_counts = HashMap::new();
        {
            let mut initial_vec = vec![0; path_count_length];
            initial_vec[0] = 1u64;
            path_counts.insert(target, initial_vec);
        }

        let mut to_visit = VecDeque::new();
        for head in self.in_edges[target].iter() {
            to_visit.push_back(*head);
        }

        while let Some(next) = to_visit.pop_front() {
            if path_counts.contains_key(&next) {
                continue; // already processed
            }

            if self.out_edges[next]
                .iter()
                .any(|head| !path_counts.contains_key(head))
            {
                // not ready to calculate yet - need to get the paths
                // to the children first
                to_visit.push_back(next);
                continue;
            }

            let mut count =
                self.out_edges[next]
                    .iter()
                    .fold(vec![0u64; path_count_length], |acc, head| {
                        let c = &path_counts[head];
                        (0..path_count_length).map(|i| c[i] + acc[i]).collect()
                    });

            if reqs.contains(&next) {
                // if this is a requirement, we should update the requirement paths
                let idx = reqs
                    .iter()
                    .enumerate()
                    .find(|(_, id)| **id == next)
                    .unwrap()
                    .0;
                count[idx + 1] = count[0];

                // if we've hit all of the requirements, we can now start populating
                // the last entry
                if (1..(path_count_length - 1)).all(|i| count[i] > 0) {
                    count[path_count_length - 1] = (1..(path_count_length - 1))
                        .map(|i| count[i])
                        .min()
                        .unwrap();
                }
            }

            path_counts.insert(next, count);

            if next == source {
                break;
            }

            for &head in self.in_edges[next].iter() {
                to_visit.push_back(head)
            }
        }

        let counts = &path_counts[&source];
        counts[path_count_length - 1]
    }
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 11: Reactor");

    let now = Instant::now();

    let reactor = read_and_parse_input();

    let part_one = reactor.solve("you", "out", vec![]);
    let part_two = reactor.solve("svr", "out", vec!["dac", "fft"]);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
