use std::time::Instant;
use std::{env, fs};

fn read_and_parse_input() -> Vec<(i64, i64)> {
    let args = env::args().collect::<Vec<_>>();

    let filepath = &args[1];

    let input = fs::read_to_string(filepath).expect(&format!("Could not read file {}", filepath));

    let locs = input
        .trim()
        .lines()
        .map(|line| {
            let mut pcs = line.split(',').map(|pc| pc.parse::<i64>().unwrap());
            (pcs.next().unwrap(), pcs.next().unwrap())
        })
        .collect::<Vec<_>>();

    locs
}

fn on_segment(p: &(i64, i64), a: &(i64, i64), b: &(i64, i64)) -> bool {
    p.0 <= a.0.max(b.0) && p.0 >= a.0.min(b.0) && p.1 <= a.1.max(b.1) && p.1 >= a.1.min(b.1)
}

fn orientation(a: &(i64, i64), b: &(i64, i64), c: &(i64, i64)) -> i64 {
    let val = (b.1 - a.1) * (c.0 - b.0) - (b.0 - a.0) * (c.1 - b.1);

    val.signum()
}

// determine if line segments ab and cd intersect
fn intersects(ab: &((i64, i64), (i64, i64)), cd: &((i64, i64), (i64, i64))) -> bool {
    let (a, b) = ab;
    let (c, d) = cd;
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);

    if o1 != o2 && o3 != o4 {
        true
    } else if o1 == 0 && on_segment(c, a, b) {
        true
    } else if o2 == 0 && on_segment(d, a, b) {
        true
    } else if o3 == 0 && on_segment(a, c, d) {
        true
    } else if o4 == 0 && on_segment(b, c, d) {
        true
    } else {
        false
    }
}

fn polygon_contains(vertices: &Vec<(i64, i64)>, pt: &(i64, i64)) -> bool {
    let min_0 = vertices.iter().map(|v| v.0).min().unwrap();
    let min_1 = vertices.iter().map(|v| v.1).min().unwrap();
    // outside of the polygon based on problem spec - can treat this as infinity
    // also conveniently oblique to the polygon edges so bypasses nonsense with
    // parallel edges
    let other = (min_0 - 1, min_1 - 1);

    let mut intersection_count = 0;
    for k in 0..vertices.len() {
        let next_k = (k + 1) % vertices.len();
        if intersects(&(*pt, other), &(vertices[k], vertices[next_k])) {
            intersection_count += 1;
        }
    }

    intersection_count % 2 > 0
}

fn solve(locs: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut part_one = 0;
    let mut part_two = 0;

    for i in 0..locs.len() {
        let loc_one = locs[i];
        for j in (1 + 1)..locs.len() {
            let loc_two = locs[j];

            let area = (loc_one.0.abs_diff(loc_two.0) + 1) * (loc_one.1.abs_diff(loc_two.1) + 1);
            let area = area as i64;
            part_one = part_one.max(area);

            if area <= part_two {
                continue;
            }

            let min_0 = loc_one.0.min(loc_two.0);
            let max_0 = loc_one.0.max(loc_two.0);
            let min_1 = loc_one.1.min(loc_two.1);
            let max_1 = loc_one.1.max(loc_two.1);

            // check that the interior is actually inside
            if !polygon_contains(&locs, &((min_0 + 1).min(max_0), (min_1 + 1).min(max_1))) {
                continue;
            }

            // if there are any interior points then this rectangle isn't valid

            let has_interior = (0..locs.len()).any(|k| {
                if k == i || k == j {
                    return false;
                }

                let loc = locs[k];
                loc.0 > min_0 && loc.0 < max_0 && loc.1 > min_1 && loc.1 < max_1
            });

            if has_interior {
                continue;
            }

            // if any line segment intersects, the retangle isn't valid. set aside incident polygon
            // edges for further analysis

            let mut incident_edges = vec![];
            let mut has_intersection = false;

            for k in 0..locs.len() {
                let next_k = (k + 1) % locs.len();
                if k == i || k == j {
                    incident_edges.push(locs[next_k]);
                    continue;
                }

                if next_k == i || next_k == j {
                    incident_edges.push(locs[k]);
                    continue;
                }

                let loc = locs[k];
                let next_loc = locs[next_k];
                if (loc.0 == min_0 || loc.0 == max_0) && (loc.1 > min_1 && loc.1 < max_1) {
                    incident_edges.push(loc);
                    continue;
                }
                if (loc.1 == min_1 || loc.1 == max_1) && (loc.0 > min_0 && loc.0 < max_0) {
                    incident_edges.push(loc);
                    continue;
                }
                if (next_loc.0 == min_0 || next_loc.0 == max_0)
                    && (next_loc.1 > min_1 && next_loc.1 < max_1)
                {
                    incident_edges.push(loc);
                    continue;
                }
                if (next_loc.1 == min_1 || next_loc.1 == max_1)
                    && (next_loc.0 > min_0 && next_loc.0 < max_0)
                {
                    incident_edges.push(loc);
                    continue;
                }

                has_intersection = intersects(&((min_0, min_1), (min_0, max_1)), &(loc, next_loc))
                    || intersects(&((min_0, min_1), (max_0, min_1)), &(loc, next_loc))
                    || intersects(&((max_0, max_1), (min_0, max_1)), &(loc, next_loc))
                    || intersects(&((max_0, max_1), (max_0, min_1)), &(loc, next_loc));

                if has_intersection {
                    break;
                }
            }

            if has_intersection {
                continue;
            }

            // for any edges that are incident, all points of incidence that are interior to the rectangle
            // must also be interior to the polygon. this represents a part of the polygon that turns so that
            // it ends up inside the rectangle

            let mut turn = false;

            for &incident in incident_edges.iter() {
                turn = ((-1)..=1)
                    .flat_map(|d_0| ((-1)..=1).map(move |d_1| (incident.0 + d_0, incident.1 + d_1)))
                    .any(|pt| {
                        if pt == incident {
                            false
                        } else if pt.0 <= min_0 || pt.0 >= max_0 || pt.1 <= min_1 || pt.1 >= max_1 {
                            false // not interior to triangle
                        } else {
                            !polygon_contains(&locs, &pt)
                        }
                    });
            }

            if turn {
                continue;
            }

            part_two = area;
        }
    }

    (part_one, part_two)
}

fn main() {
    println!("Advent of Code 2025");
    println!("Day 9: Movie Theater");

    let now = Instant::now();

    let locs = read_and_parse_input();

    let (part_one, part_two) = solve(&locs);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
