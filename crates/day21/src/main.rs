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
                next_chars.extend(num_lookup[nix(c1)][nix(c2)].iter());
            });
            println!("robot 1 {:?}", next_chars.iter().join(""));
            next_chars.insert(0, 'A');

            let mut next_next_chars: Vec<char> = vec![];
            next_chars.into_iter().tuple_windows().for_each(|(c1, c2)| {
                next_next_chars.extend(dir_lookup[dix(c1)][dix(c2)].iter());
            });
            println!("robot 2 {:?}", next_next_chars.iter().join(""));
            next_next_chars.insert(0, 'A');

            let mut next_next_next_chars: Vec<char> = vec![];
            next_next_chars
                .into_iter()
                .tuple_windows()
                .for_each(|(c1, c2)| {
                    next_next_next_chars.extend(dir_lookup[dix(c1)][dix(c2)].iter());
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

fn nix(c: char) -> usize {
    c.to_digit(11).unwrap() as usize
}
const fn dix(c: char) -> usize {
    match c {
        '<' => 0,
        '>' => 1,
        'v' => 2,
        '^' => 3,
        'A' => 4,
        _ => panic!(),
    }
}

fn dix_r(c: usize) -> char {
    match c {
        0 => '<',
        1 => '>',
        2 => 'v',
        3 => '^',
        4 => 'A',
        _ => panic!(),
    }
}

const DKN: [&[char]; 5] = [
    &['v'],           // '<' => 0,
    &['v', 'A'],      // '>' => 1,
    &['<', '^', '>'], // 'v' => 2,
    &['v', 'A'],      // '^' => 3,
    &['^', '>'],      // 'A' => 4,
];

const NKN: [&[char]; 11] = [
    &['2', 'A'],           // '0'
    &['2', '4'],           // '1'
    &['1', '0', '3', '5'], // '2'
    &['A', '2', '6'],      // '3'
    &['1', '5', '7'],      // '4'
    &['2', '4', '6', '8'], // '5'
    &['3', '5', '9'],      // '6'
    &['4', '8'],           // '7'
    &['5', '7', '9'],      // '8'
    &['6', '8'],           // '9'
    &['0', '3'],           // 'A'
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
                        min_cost[dix(org)] = 0;
                        let mut heap = BinaryHeap::new();
                        heap.push(Reverse((0, org)));
                        while let Some(Reverse((c, pos))) = heap.pop() {
                            if pos == dst {
                                break;
                            }
                            if c > min_cost[dix(pos)] {
                                continue;
                            }
                            for n in DKN[dix(pos)] {
                                let new_cost =
                                    c + cost[dix(pos)][dix(*n)] + cost[dix(*n)][dix('A')];
                                if new_cost < min_cost[dix(*n)] {
                                    heap.push(Reverse((new_cost, *n)));
                                    min_cost[dix(*n)] = new_cost;
                                }
                            }
                        }

                        let orig = dix(org);
                        let dest = dix(dst);
                        cost_n[orig][dest] = min_cost[dix(dst)];
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

fn compute_distance_numeric_kp(cost: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let m = arr2(&[
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['X', '0', 'A'],
    ]);
    let mut cost_n: Vec<Vec<usize>> = vec![vec![1_usize; 11]; 11];
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
                    min_cost[dix(org)] = 0;
                    let mut heap = BinaryHeap::new();
                    heap.push(Reverse((0, org)));
                    while let Some(Reverse((c, pos))) = heap.pop() {
                        if pos == dst {
                            break;
                        }
                        if c > min_cost[dix(pos)] {
                            continue;
                        }
                        for n in DKN[dix(pos)] {
                            let new_cost = c + cost[dix(pos)][dix(*n)] + cost[dix(*n)][dix('A')];
                            if new_cost < min_cost[dix(*n)] {
                                heap.push(Reverse((new_cost, *n)));
                                min_cost[dix(*n)] = new_cost;
                            }
                        }
                    }

                    let orig = dix(org);
                    let dest = dix(dst);
                    cost_n[orig][dest] = min_cost[dix(dst)];
                }
            }
        }
    }
    println!("{cost:?}");
    cost_n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_numeric_kp() {
        let lookup = compute_sequences_numeric_kp();
        for c in ['A', '1', '4', '7', '9'] {
            assert_eq!(&vec!['A'], &lookup[nix(c)][nix(c)]);
        }
        assert_eq!(&vec!['<', 'A'], &lookup[nix('A')][nix('0')]);
        assert_eq!(&vec!['^', '<', 'A'], &lookup[nix('A')][nix('2')]);
        assert_eq!(&vec!['>', 'v', 'A'], &lookup[nix('7')][nix('5')]);
    }

    #[test]
    fn test_compute_distance_directional_kp_1() {
        let lookup = compute_distance_directional_kp(1);
        for c in ['A', '<', '^', '>', 'v'] {
            assert_eq!(0, lookup[dix(c)][dix(c)]);
        }
        assert_eq!(2, lookup[dix('A')][dix('>')]);
        assert_eq!(2, lookup[dix('A')][dix('^')]);
        assert_eq!(4, lookup[dix('A')][dix('v')]);
        assert_eq!(6, lookup[dix('A')][dix('<')]);
    }

    #[test]
    fn test_compute_distance_directional_kp_2() {
        let lookup = compute_distance_directional_kp(2);
        for c in ['A', '<', '^', '>', 'v'] {
            assert_eq!(0, lookup[dix(c)][dix(c)]);
        }
        assert_eq!(10, lookup[dix('A')][dix('v')]);
    }
}
