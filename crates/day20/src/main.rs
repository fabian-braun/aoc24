use anyhow::Context;
use maplit::{hashmap, hashset};
use ndarray::Axis;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;
use utilities::M;

const VERSION: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    let day = VERSION
        .strip_prefix("day")
        .unwrap_or_default()
        .parse()
        .unwrap_or(1);
    let content = utilities::get_example(day).await;
    println!("Example Solution for day {}: \n{:?}\n", day, run(content));
    // let content = utilities::get_input(day).await;
    // let start = Instant::now();
    // let solution = run(content);
    // let time_taken = start.elapsed();
    // println!(
    //     "Actual Solution for day {}: \n{:?}\nin time {:?}",
    //     day, solution, time_taken
    // );
}

fn run(input: String) -> anyhow::Result<String> {
    let (m, start, end) = char_matrix(input)?;

    // print(&m);
    let mut min_cost = hashmap! { start => 0_u64 };
    let mut pred: HashMap<(usize, usize), (usize, usize)> = hashmap! {};
    let mut heap = BinaryHeap::<Reverse<(u64, (usize, usize))>>::new();
    heap.push(Reverse((0u64, start)));
    while let Some(Reverse((cost, pos))) = heap.pop() {
        if cost > min_cost[&pos] {
            continue;
        }
        let new_cost: u64 = cost + 1;
        neighbours(pos).into_iter().for_each(|n| {
            if new_cost < *min_cost.get(&n).unwrap_or(&u64::MAX) && (m[n] == '.' || m[n] == 'E') {
                heap.push(Reverse((new_cost, n)));
                min_cost.insert(n, new_cost);
                pred.insert(n, pos);
            }
        })
    }

    let mut s_p = vec![];
    let mut curr = end;
    while let Some(c) = pred.get(&curr) {
        s_p.push(*c);
        curr = *c
    }
    s_p.reverse();

    let mut saved = vec![0_usize; (min_cost[&end] + 1) as usize];
    s_p.iter().for_each(|&s_p_pos| {
        // try to find short-cut from pos and compute saved time
        find_shortcuts(s_p_pos, &min_cost, &m, &mut saved);
    });

    dbg!(&saved[50..]);
    let result: usize = saved.iter().skip(50).sum();

    Ok(result.to_string())
}

fn find_shortcuts(
    s_p_pos: (usize, usize),
    original_min_cost: &HashMap<(usize, usize), u64>,
    m: &M,
    saved: &mut [usize],
) {
    let start_cost = original_min_cost[&s_p_pos];
    let mut min_cost = hashmap! { s_p_pos => start_cost };
    let mut heap = BinaryHeap::<Reverse<(u64, (usize, usize))>>::new();
    heap.push(Reverse((start_cost, s_p_pos)));
    while let Some(Reverse((cost, pos))) = heap.pop() {
        if cost > min_cost[&pos] || cost > start_cost + 20 {
            continue;
        }
        let new_cost: u64 = cost + 1;
        neighbours(pos).into_iter().for_each(|n| {
            if new_cost < *min_cost.get(&n).unwrap_or(&u64::MAX) {
                if m[n] == '.' || m[n] == 'E' {
                    // we're on a good path again
                    if new_cost < original_min_cost[&n] {
                        // we found a shortcut
                        let saved_cost = original_min_cost[&n] - new_cost;
                        saved[saved_cost as usize] += 1;
                        min_cost.insert(n, new_cost);
                    }
                } else if m[n] == '#' {
                    heap.push(Reverse((new_cost, n)));
                    min_cost.insert(n, new_cost);
                }
            }
        })
    }
}

fn neighbours(x: (usize, usize)) -> [(usize, usize); 4] {
    [
        (x.0, x.1 + 1),
        (x.0, x.1 - 1),
        (x.0 + 1, x.1),
        (x.0 - 1, x.1),
    ]
}
fn print(m: &M) {
    for y in 0..m.len_of(Axis(0)) {
        println!();
        for x in 0..m.len_of(Axis(1)) {
            print!("{}", m[(y, x)]);
        }
    }
    println!();
}

fn char_matrix(raw: String) -> anyhow::Result<(M, (usize, usize), (usize, usize))> {
    let y_len = raw.lines().filter(|line| !line.is_empty()).count();
    let x_len = raw.lines().next().context("char_matrix")?.len();
    let mut m = M::default((y_len, x_len));
    let mut start = (0, 0);
    let mut end = (0, 0);

    raw.lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if x == 0 || y == 0 || x == x_len - 1 || y == y_len - 1 {
                    if c != '#' {
                        panic!()
                    }
                    m[(y, x)] = 'O';
                } else {
                    if c == 'S' {
                        start = (y, x);
                        m[(y, x)] = c;
                    } else if c == 'E' {
                        end = (y, x);
                        m[(y, x)] = c;
                    } else {
                        m[(y, x)] = c;
                    }
                }
            })
        });
    if start == (0, 0) {
        panic!("start not found")
    }
    if end == (0, 0) {
        panic!("end not found")
    }
    Ok((m, start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        struct Example {
            content: &'static str,
            expected: &'static str,
        }
        let examples = [
            Example {
                content: "1",
                expected: "1",
            },
            Example {
                content: "1",
                expected: "1",
            },
        ];
        for (i, ex) in examples.iter().enumerate() {
            assert_eq!(
                ex.expected.to_string(),
                run(ex.content.to_string()).unwrap(),
                "example {} failed:",
                i + 1
            );
        }
    }
}
