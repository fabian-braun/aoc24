use itertools::Itertools;
use ndarray::{arr2, Axis};
use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;
use std::iter::Rev;
use std::time::Instant;

const VERSION: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    let day = VERSION
        .strip_prefix("day")
        .unwrap_or_default()
        .parse()
        .unwrap_or(1);
    let content = utilities::get_example(day).await;
    let ex_solution = run(content).unwrap();
    println!("Example Solution for day {}: \n{:?}\n", day, ex_solution);
    assert_eq!("126384".to_string(), ex_solution);
    let content = utilities::get_input(day).await;
    let start = Instant::now();
    let solution = run(content).unwrap();
    assert_ne!("181330".to_string(), solution);
    assert_ne!("184390".to_string(), solution);
    let time_taken = start.elapsed();
    println!(
        "Actual Solution for day {}: \n{:?}\nin time {:?}",
        day, solution, time_taken
    );
}

fn run(input: String) -> anyhow::Result<String> {
    let num_lookup = compute_sequences_numeric_kp();
    let dir_lookup = compute_sequences_directional_kp();
    let result: usize = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            println!("{:?}", line);
            let num_part: usize = line.strip_suffix('A').unwrap().parse().unwrap();
            let mut chars = line.chars().collect_vec();
            chars.insert(0, 'A');
            let mut next_chars: Vec<char> = vec![];
            chars.into_iter().tuple_windows().for_each(|(c1, c2)| {
                next_chars.extend(num_lookup[num_idx(c1)][num_idx(c2)].iter());
            });
            println!("robot 1 {:?}", next_chars.iter().join(""));
            next_chars.insert(0, 'A');

            let mut next_next_chars: Vec<char> = vec![];
            next_chars.into_iter().tuple_windows().for_each(|(c1, c2)| {
                next_next_chars.extend(dir_lookup[dir_idx(c1)][dir_idx(c2)].iter());
            });
            println!("robot 2 {:?}", next_next_chars.iter().join(""));
            next_next_chars.insert(0, 'A');

            let mut next_next_next_chars: Vec<char> = vec![];
            next_next_chars
                .into_iter()
                .tuple_windows()
                .for_each(|(c1, c2)| {
                    next_next_next_chars.extend(dir_lookup[dir_idx(c1)][dir_idx(c2)].iter());
                });
            println!("robot 3 {:?}", next_next_next_chars.iter().join(""));
            println!(
                "{} * {} = {}",
                next_next_next_chars.len(),
                num_part,
                next_next_next_chars.len() * num_part
            );
            next_next_next_chars.len() * num_part
        })
        .sum();

    Ok(result.to_string())
}

fn num_idx(c: char) -> usize {
    c.to_digit(11).unwrap() as usize
}
const fn dir_idx(c: char) -> usize {
    match c {
        '<' => 0,
        '>' => 1,
        'v' => 2,
        '^' => 3,
        'A' => 4,
        _ => panic!(),
    }
}
fn dir_idx_rev(c: usize) -> char {
    match c {
        0 => '<',
        1 => '>',
        2 => 'v',
        3 => '^',
        4 => 'A',
        _ => panic!(),
    }
}

fn compute_sequences_numeric_kp() -> Vec<Vec<Vec<char>>> {
    let m = arr2(&[
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['X', '0', 'A'],
    ]);
    let mut sequences: Vec<Vec<Vec<char>>> = vec![vec![vec![]; 11]; 11];
    for y_org in 0..m.len_of(Axis(0)) {
        for x_org in 0..m.len_of(Axis(1)) {
            for y_dst in 0..m.len_of(Axis(0)) {
                for x_dst in 0..m.len_of(Axis(1)) {
                    if m[(y_org, x_org)] == 'X' || m[(y_dst, x_dst)] == 'X' {
                        continue;
                    }
                    let orig = num_idx(m[(y_org, x_org)]);
                    let dest = num_idx(m[(y_dst, x_dst)]);
                    let mut dy: isize = y_dst as isize - y_org as isize;
                    let mut dx: isize = x_dst as isize - x_org as isize;
                    let mut v = vec![];
                    let mut h = vec![];
                    while dy > 0 {
                        v.push('v');
                        dy -= 1;
                    }
                    while dx < 0 {
                        h.push('<');
                        dx += 1;
                    }
                    while dy < 0 {
                        v.push('^');
                        dy += 1;
                    }
                    while dx > 0 {
                        h.push('>');
                        dx -= 1;
                    }
                    if x_org == 0 && x_dst != 0 {
                        sequences[orig][dest].extend(h);
                        sequences[orig][dest].extend(v);
                    } else {
                        sequences[orig][dest].extend(v);
                        sequences[orig][dest].extend(h);
                    }
                    sequences[orig][dest].push('A');
                }
            }
        }
    }

    sequences
}

const KPN: [&[char]; 5] = [
    &['v'],           // '<' => 0,
    &['v', 'A'],      // '>' => 1,
    &['<', '^', '>'], // 'v' => 2,
    &['v', 'A'],      // '^' => 3,
    &['^', '>'],      // 'A' => 4,
];

fn compute_distance_directional_kp(depth: usize) -> Vec<Vec<usize>> {
    let m = arr2(&[['X', '^', 'A'], ['<', 'v', '>']]);
    let mut cost: Vec<Vec<usize>> = vec![vec![1_usize; 5]; 5];
    let mut cost_n: Vec<Vec<usize>> = vec![vec![1_usize; 5]; 5];
    for _ in 0..depth {
        for y_org in 0..m.len_of(Axis(0)) {
            for x_org in 0..m.len_of(Axis(1)) {
                for y_dst in 0..m.len_of(Axis(0)) {
                    for x_dst in 0..m.len_of(Axis(1)) {
                        let org = m[(y_org, x_org)];
                        let dst = m[(y_dst, x_dst)];
                        if org == 'X' || dst == 'X' {
                            continue;
                        }
                        let mut min_cost = vec![usize::MAX; 5];
                        min_cost[dir_idx(org)] = 0;
                        let mut heap = BinaryHeap::new();
                        heap.push(Reverse((0, org)));
                        while let Some(Reverse((c, pos))) = heap.pop() {
                            if pos == dst {
                                break;
                            }
                            if c > min_cost[dir_idx(pos)] {
                                continue;
                            }
                            for n in KPN[dir_idx(pos)] {
                                let new_cost = c
                                    + cost[dir_idx(pos)][dir_idx(*n)]
                                    + cost[dir_idx(*n)][dir_idx('A')];
                                if new_cost < min_cost[dir_idx(*n)] {
                                    heap.push(Reverse((new_cost, *n)));
                                    min_cost[dir_idx(*n)] = new_cost;
                                }
                            }
                        }

                        let orig = dir_idx(org);
                        let dest = dir_idx(dst);
                        cost_n[orig][dest] = min_cost[dir_idx(dst)];
                    }
                }
            }
        }
        cost = cost_n;
        println!("{cost:?}");
        cost_n = vec![vec![1_usize; 5]; 5];
    }
    cost
}

fn compute_sequences_directional_kp() -> Vec<Vec<Vec<char>>> {
    let m = arr2(&[['X', '^', 'A'], ['<', 'v', '>']]);
    let mut sequences: Vec<Vec<Vec<char>>> = vec![vec![vec![]; 5]; 5];
    for y_org in 0..m.len_of(Axis(0)) {
        for x_org in 0..m.len_of(Axis(1)) {
            for y_dst in 0..m.len_of(Axis(0)) {
                for x_dst in 0..m.len_of(Axis(1)) {
                    if m[(y_org, x_org)] == 'X' || m[(y_dst, x_dst)] == 'X' {
                        continue;
                    }
                    let orig = dir_idx(m[(y_org, x_org)]);
                    let dest = dir_idx(m[(y_dst, x_dst)]);
                    let mut dy: isize = y_dst as isize - y_org as isize;
                    let mut dx: isize = x_dst as isize - x_org as isize;
                    let mut v = vec![];
                    let mut h = vec![];
                    while dx > 0 {
                        h.push('>');
                        dx -= 1;
                    }
                    while dy > 0 {
                        v.push('v');
                        dy -= 1;
                    }
                    while dy < 0 {
                        v.push('^');
                        dy += 1;
                    }
                    while dx < 0 {
                        h.push('<');
                        dx += 1;
                    }
                    if y_org == 0 && y_dst != 0 {
                        sequences[orig][dest].extend(v);
                        sequences[orig][dest].extend(h);
                    } else {
                        sequences[orig][dest].extend(h);
                        sequences[orig][dest].extend(v);
                    }
                    sequences[orig][dest].push('A');
                }
            }
        }
    }

    sequences
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_numeric_kp() {
        let lookup = compute_sequences_numeric_kp();
        for c in ['A', '1', '4', '7', '9'] {
            assert_eq!(&vec!['A'], &lookup[num_idx(c)][num_idx(c)]);
        }
        assert_eq!(&vec!['<', 'A'], &lookup[num_idx('A')][num_idx('0')]);
        assert_eq!(&vec!['^', '<', 'A'], &lookup[num_idx('A')][num_idx('2')]);
        assert_eq!(&vec!['>', 'v', 'A'], &lookup[num_idx('7')][num_idx('5')]);
    }

    #[test]
    fn test_directional_kp() {
        let lookup = compute_sequences_directional_kp();
        for c in ['A', '<', '^', '>', 'v'] {
            assert_eq!(&vec!['A'], &lookup[dir_idx(c)][dir_idx(c)]);
        }
        assert_eq!(&vec!['<', 'A'], &lookup[dir_idx('A')][dir_idx('^')]);
        assert_eq!(&vec!['v', '<', 'A'], &lookup[dir_idx('^')][dir_idx('<')]);
        assert_eq!(&vec!['>', '^', 'A'], &lookup[dir_idx('<')][dir_idx('^')]);
    }

    #[test]
    fn test_compute_distance_directional_kp() {
        let lookup = compute_distance_directional_kp(1);
        for c in ['A', '<', '^', '>', 'v'] {
            assert_eq!(0, lookup[dir_idx(c)][dir_idx(c)]);
        }
        assert_eq!(2, lookup[dir_idx('A')][dir_idx('>')]);
        assert_eq!(2, lookup[dir_idx('A')][dir_idx('^')]);
        assert_eq!(4, lookup[dir_idx('A')][dir_idx('v')]);
        assert_eq!(6, lookup[dir_idx('A')][dir_idx('<')]);
    }
}
