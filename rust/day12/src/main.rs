use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use std::{env, fs, u64};

fn read_and_parse_input() -> (Vec<Vec<Vec<bool>>>, Vec<((usize, usize), Vec<usize>)>) {
    let args = env::args().collect::<Vec<_>>();

    let filepath = &args[1];

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    let chunks = input.trim().split("\n\n").collect::<Vec<_>>();

    let presents = chunks[0..(chunks.len() - 1)]
        .iter()
        .map(|chunk| {
            chunk
                .trim()
                .lines()
                .skip(1)
                .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let trees = chunks[chunks.len() - 1]
        .trim()
        .lines()
        .map(|line| {
            let mut pcs = line.split(":");
            let mut dims = pcs.next().unwrap().split("x");
            let dims = (
                dims.next().unwrap().parse::<usize>().unwrap(),
                dims.next().unwrap().parse::<usize>().unwrap(),
            );
            let reqs = pcs
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|req| req.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (dims, reqs)
        })
        .collect::<Vec<_>>();

    (presents, trees)
}

fn transform(present: &Vec<Vec<bool>>) -> Vec<HashSet<(usize, usize)>> {
    assert!(present.len() == 3);
    assert!(present[0].len() == 3);

    let mut result = vec![present.clone()];

    let mut pts = present
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, &b)| {
                if b {
                    Some((r as i32 - 1, c as i32 - 1))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();

    // generate rotations
    for _ in 0..3 {
        pts = pts.iter().map(|(r, c)| (*c, -*r)).collect();
        let rot = (0..3)
            .map(|r| {
                (0..3)
                    .map(|c| pts.contains(&(r as i32 - 1, c as i32 - 1)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        result.push(rot);
    }

    for i in 0..4 {
        let p = &result[i];
        let flipped = vec![p[2].clone(), p[1].clone(), p[0].clone()];
        result.push(flipped);
    }

    // eliminate equivalent entires
    let result = result.into_iter().collect::<HashSet<_>>();

    result
        .into_iter()
        .map(|rot| {
            rot.iter()
                .enumerate()
                .flat_map(|(r, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(move |(c, b)| if *b { Some((r, c)) } else { None })
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>()
}

fn can_fit_old(
    tree: &((usize, usize), Vec<usize>),
    presents: &Vec<Vec<HashSet<(usize, usize)>>>,
) -> bool {
    let (num_rows, num_cols) = tree.0;
    let reqs = &tree.1;
    let num_presents = reqs.len();

    let mut to_visit = vec![];

    to_visit.push((
        (0usize, 0usize),
        HashSet::<(usize, usize)>::new(),
        vec![0; num_presents],
    ));

    while let Some(next) = to_visit.pop() {
        let ((r, c), map, placed) = next;
        // println!("visiting {}, {}", r, c);

        if r > num_rows - 3 {
            continue;
        }

        let next_point = if c < num_cols - 3 {
            (r, c + 1)
        } else {
            (r + 1, 0)
        };

        // add a node that corresponds to not filling this space
        // (unless we'd leave the top line completely empty)
        if next_point != (1, 0) || placed.iter().any(|i| *i > 0) {
            to_visit.push((next_point, map.clone(), placed.clone()));
        }

        if map.contains(&(r, c)) {
            continue;
        }

        for (p, rots) in presents.iter().enumerate() {
            if placed[p] == reqs[p] {
                continue;
            }

            for rot in rots.iter() {
                let filled = rot
                    .iter()
                    .any(|(d_r, d_c)| map.contains(&(r + d_r, c + d_c)));
                if filled {
                    continue;
                }

                let mut new_map = map.clone();
                for (d_r, d_c) in rot {
                    new_map.insert((r + d_r, c + d_c));
                }

                // if new_map
                //     .iter()
                //     .any(|(rr, cc)| *rr >= num_rows || *cc >= num_cols)
                // {
                //     println!("{:?}", new_map);
                //     println!("{}, {}", num_rows, num_cols);
                //     println!("{}, {}", r, c);
                //     panic!()
                // }

                let mut new_placed = placed.clone();
                new_placed[p] += 1;
                if &new_placed == reqs {
                    println!("return");
                    return true;
                }

                to_visit.push((next_point, new_map, new_placed));
            }
        }
    }

    println!("return");
    false
}

fn can_fit(tree: &((usize, usize), Vec<usize>), present_areas: &Vec<usize>) -> bool {
    let ((width, height), reqs) = tree;

    let total_presents = reqs.iter().cloned().sum::<usize>();

    let present_area = reqs
        .iter()
        .enumerate()
        .map(|(i, qty)| qty * present_areas[i])
        .sum::<usize>();

    if (width / 3) * (height / 3) >= total_presents {
        // can fit each into a 3x3 box
        true
    } else if width * height < present_area {
        false // not enough room to fit everything
    } else {
        unimplemented!()
    }
}

fn solve(presents: &Vec<Vec<Vec<bool>>>, trees: &Vec<((usize, usize), Vec<usize>)>) -> usize {
    // let presents = presents
    //     .iter()
    //     .map(|present| transform(present))
    //     .collect::<Vec<_>>();

    let present_areas = presents
        .iter()
        .map(|present| {
            present
                .iter()
                .map(|row| row.iter().filter(|&&b| b).count())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    trees.iter().filter(|&t| can_fit(t, &present_areas)).count()
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 12: Christmas Tree Farm");

    let now = Instant::now();

    let (presents, trees) = read_and_parse_input();

    let part_one = solve(&presents, &trees);

    println!("Part one: {}", part_one);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
