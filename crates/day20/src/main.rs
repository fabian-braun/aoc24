use anyhow::Context;
use itertools::Itertools;
use maplit::hashmap;
use ndarray::Axis;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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

    let mut s_p = vec![end];
    let mut curr = end;
    while let Some(c) = pred.get(&curr) {
        s_p.push(*c);
        curr = *c
    }
    s_p.reverse();

    let mut saved = vec![0_usize; min_cost[&end] as usize];
    s_p.iter().for_each(|&o| {
        s_p.iter().for_each(|&d| {
            if min_cost[&o] < min_cost[&d] {
                let md = d.1.abs_diff(o.1) + d.0.abs_diff(o.0);
                let original_cost = (min_cost[&d] - min_cost[&o]) as usize;
                if original_cost > md && md < 21 {
                    saved[original_cost - md] += 1;
                }
            }
        });
    });

    // let saved_non_0 = saved
    //     .iter()
    //     .enumerate()
    //     .skip(100)
    //     .filter_map(|(idx, val)| (val > &0).then_some((idx, val)))
    //     .collect_vec();
    // dbg!(saved_non_0);
    let result: usize = saved.iter().skip(100).sum();
    // wrong results submitted to AOC
    assert_ne!(43143107, result);

    Ok(result.to_string())
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
