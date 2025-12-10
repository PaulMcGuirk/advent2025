use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::{env, fs, u64};

struct Machine {
    light_target: u32,
    buttons: Vec<u32>,
    press_targets: Vec<u32>,
}

fn read_and_parse_input() -> Vec<Machine> {
    let args = env::args().collect::<Vec<_>>();

    let filepath = &args[1];

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    let locs = input
        .trim()
        .lines()
        .map(|line| {
            let pcs = line.split_ascii_whitespace().collect::<Vec<_>>();
            let light_target = pcs[0]
                .chars()
                .skip(1)
                .enumerate()
                .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
                .sum::<u32>();

            let buttons = pcs[1..(pcs.len() - 1)]
                .iter()
                .map(|switch| {
                    switch[1..(switch.len() - 1)]
                        .split(',')
                        .fold(0, |acc, seg| {
                            let seg = seg.parse::<u32>().unwrap();
                            acc | (1 << seg)
                        })
                })
                .collect::<Vec<_>>();

            let press_targets = pcs[pcs.len() - 1];
            let press_targets = press_targets[1..(press_targets.len() - 1)]
                .split(',')
                .map(|target| target.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            Machine {
                light_target,
                buttons,
                press_targets,
            }
        })
        .collect::<Vec<_>>();

    locs
}

impl Machine {
    fn solve_part_one(&self) -> u32 {
        let mut seen = HashSet::new();

        let mut to_visit = VecDeque::new();
        to_visit.push_back((0, 0));

        while let Some(next) = to_visit.pop_front() {
            let (state, depth) = next;
            if seen.contains(&state) {
                continue;
            }
            seen.insert(state);

            if state == self.light_target {
                return depth;
            }

            for button in self.buttons.iter() {
                to_visit.push_back((state ^ button, depth + 1));
            }
        }

        panic!()
    }

    fn solve_part_two(&self) -> u64 {
        let num_targets = self.press_targets.len();

        // lifts[i] is the button values that will increase slot i
        let lifts = (0..num_targets)
            .map(|i| {
                self.buttons
                    .iter()
                    .filter_map(|&button| {
                        if ((1 << i) & button) > 0 {
                            Some(button)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut allowed_buttons = self.buttons.iter().cloned().collect::<HashSet<_>>();

        // this will be a list of lists of trial runs indicating the number of times
        // that we've pressed each button
        let mut combos = vec![(vec![0; num_targets], 0u64)];

        // fill up each slot 1 by 1
        let mut to_fill = (0..num_targets).collect::<HashSet<_>>();
        loop {
            if to_fill.is_empty() {
                break;
            }

            // the slot that we'll fill this one is the one with the least number
            // of available slots. in the even of a tie, the one with the smaller
            // fill amount
            let target = to_fill
                .iter()
                .map(|&i| {
                    let lifts = lifts[i]
                        .iter()
                        .filter(|b| allowed_buttons.contains(&b))
                        .count();
                    (i, lifts, self.press_targets[i])
                })
                .min_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)))
                .unwrap()
                .0;
            to_fill.remove(&target);

            let allowed_lifts = lifts[target]
                .iter()
                .filter(|b| allowed_buttons.contains(&b))
                .collect::<Vec<_>>();

            let mut new_combos = vec![];

            // for each combo that we've built so far, extend in all the ways that we can
            // hit the remaining target
            for (combo, count) in combos.iter() {
                if combo[target] == self.press_targets[target] {
                    new_combos.push((combo.clone(), *count));
                    continue;
                }

                if allowed_lifts.len() == 0 {
                    continue;
                }

                if combo[target] > self.press_targets[target] {
                    // we should have filtered this out
                    unreachable!()
                }

                let rest = self.press_targets[target] - combo[target];

                // lift counts will be a list of all of the different ways
                // to lift the target we're working on to the goal
                let mut lift_counts = vec![vec![0u32; allowed_lifts.len()]];

                for lift_idx in 1..allowed_lifts.len() {
                    let mut new_lift_counts: Vec<Vec<u32>> = vec![];
                    for lift_count in lift_counts.iter() {
                        let rest_rest = rest - lift_count.iter().sum::<u32>();
                        for press in 0..=rest_rest {
                            let mut new_lift_count = lift_count.clone();
                            new_lift_count[lift_idx] = press;
                            new_lift_counts.push(new_lift_count);
                        }
                    }
                    lift_counts = new_lift_counts;
                }

                for lift_count in lift_counts.iter_mut() {
                    let rest_rest = rest - lift_count.iter().sum::<u32>();
                    lift_count[0] = rest_rest;
                }

                let new_count = count + rest as u64;

                // for each of the lift counts, fold that into the combo,
                // making sure to handle any side effects
                'lift_count_loop: for lift_count in lift_counts.iter() {
                    let mut new_combo = combo.clone();
                    for (lift_idx, lift_qty) in lift_count.iter().enumerate() {
                        let button = allowed_lifts[lift_idx];
                        for t in 0..num_targets {
                            if ((1 << t) & button) > 0 {
                                new_combo[t] += lift_qty;
                                if new_combo[t] > self.press_targets[t] {
                                    continue 'lift_count_loop;
                                }
                            }
                        }
                    }
                    new_combos.push((new_combo, new_count));
                }
            }

            if new_combos.len() == 0 {
                panic!("Can't solve")
            }
            combos = new_combos;

            // we can never raise target again, so take that out of consideration
            // to reduce complexity
            allowed_buttons = allowed_buttons
                .iter()
                .cloned()
                .filter(|b| !allowed_lifts.contains(&b))
                .collect();
        }

        // println!("{:?}", combos);

        *combos
            .iter()
            .filter_map(|(combo, pushes)| {
                if (0..num_targets).all(|i| combo[i] == self.press_targets[i]) {
                    Some(pushes)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }
}

fn solve_part_one(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(|m| m.solve_part_one()).sum()
}

fn solve_part_two(machines: &Vec<Machine>) -> u64 {
    machines.iter().map(|m| m.solve_part_two()).sum()
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 9: Factory");

    let now = Instant::now();

    let machines = read_and_parse_input();

    let part_one = solve_part_one(&machines);
    let part_two = solve_part_two(&machines);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
