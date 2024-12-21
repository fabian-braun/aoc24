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
    let content = utilities::get_input(day).await;
    let start = Instant::now();
    let solution = run(content);
    let time_taken = start.elapsed();
    println!(
        "Actual Solution for day {}: \n{:?}\nin time {:?}",
        day, solution, time_taken
    );
}

fn run(input: String) -> anyhow::Result<String> {
    let (m, start, end) = char_matrix(input)?;

    print(&m);
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

    let mut s_p = vec![end];
    let mut curr = end;
    while let Some(c) = pred.get(&curr) {
        s_p.push(*c);
        curr = *c
    }
    s_p.reverse();

    let saved = &mut vec![0_usize; (min_cost[&end] + 1) as usize];
    s_p.iter().for_each(|&s_p_pos| {
        // try to find short-cut from pos and compute saved time
        neighbours(s_p_pos).into_iter().for_each(|n1| {
            if m[n1] == '#' {
                neighbours(n1).into_iter().for_each(|n2| {
                    if n2 != s_p_pos && (m[n2] == '.' || m[n2] == 'E') {
                        let cost_origin = min_cost[&s_p_pos];
                        let cost_destination = min_cost[&n2];
                        if cost_origin + 2 < cost_destination {
                            let saved_cost = cost_destination - 2 - cost_origin;
                            saved[saved_cost as usize] += 1;
                        }
                    }
                })
            }
        });
    });

    // dbg!(&saved);
    let result: usize = saved.iter().skip(100).sum();

    Ok(result.to_string())
}

fn dfs(
    pos: (usize, usize),
    cost: u64,
    cost_bound: u64,
    mut cheat_rem: u8,
    min_cost: &HashMap<(usize, usize), u64>,
    visited: &mut HashSet<(usize, usize)>,
    m: &M,
    saved: &mut [usize],
) {
    if cost > *min_cost.get(&pos).unwrap_or(&cost_bound) || visited.contains(&pos) {
        return;
    }
    if m[pos] == 'E' {
        if cost < cost_bound {
            saved[(cost_bound - cost) as usize] += 1;
        }
        return;
    }
    visited.insert(pos);
    neighbours(pos).into_iter().for_each(|n| {
        if m[n] == '.' || m[n] == 'E' {
            if cheat_rem == 1 {
                cheat_rem = cheat_rem - 1;
            };
            dfs(
                n,
                cost + 1,
                cost_bound,
                cheat_rem,
                min_cost,
                visited,
                m,
                saved,
            )
        } else if cheat_rem > 0 && m[n] == '#' {
            cheat_rem = cheat_rem - 1;
            dfs(
                n,
                cost + 1,
                cost_bound,
                cheat_rem,
                min_cost,
                visited,
                m,
                saved,
            )
        }
    });
    assert!(visited.remove(&pos));
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
