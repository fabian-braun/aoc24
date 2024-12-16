use maplit::{hashmap, hashset};
use ndarray::Axis;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;
use utilities::{char_matrix, I, M};

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

const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (0, -1);
const EAST: (isize, isize) = (0, 1);

fn turn(d: (isize, isize)) -> [(isize, isize); 2] {
    match d {
        NORTH => [WEST, EAST],
        SOUTH => [WEST, EAST],
        WEST => [NORTH, SOUTH],
        EAST => [NORTH, SOUTH],
        _ => panic!("Oh no"),
    }
}

fn run(input: String) -> anyhow::Result<String> {
    let m = char_matrix(input)?;
    let y_len = m.len_of(Axis(0));
    let x_len = m.len_of(Axis(1));
    let mut c_m = I::default((y_len, x_len));
    let start: (u64, (usize, usize), (isize, isize)) = (0, (y_len - 2, 1), EAST);
    let end = (1, x_len - 2);
    println!("search path from {start:?} to {end:?}");
    let mut min_cost = hashmap! { start.1 => start.0 };
    let mut pred: HashMap<(usize, usize), (usize, usize)> = hashmap! {};
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(start));
    while let Some(Reverse((cost, pos, d))) = heap.pop() {
        if cost > min_cost[&pos] {
            continue;
        }
        neighbours(cost, pos, d).into_iter().for_each(|n| {
            if n.0 < *min_cost.get(&n.1).unwrap_or(&u64::MAX) && (m[n.1] == '.' || m[n.1] == 'E') {
                heap.push(Reverse(n));
                min_cost.insert(n.1, n.0);
                c_m[n.1] = n.0 as usize;
                pred.insert(n.1, pos);
            }
        })
    }
    // print(&m, &c_m);

    let mut s_p = hashset![end];
    let mut curr = end;
    while let Some(c) = pred.get(&curr) {
        s_p.insert(*c);
        curr = *c
    }
    let mut result = count_tiles(start, &min_cost, &m);
    result.insert(start.1);
    Ok(result.len().to_string())
}

fn count_tiles(
    current: (u64, (usize, usize), (isize, isize)),
    min_cost: &HashMap<(usize, usize), u64>,
    m: &M,
) -> HashSet<(usize, usize)> {
    let res = if current.0 > min_cost[&current.1] + 2000 {
        hashset! {}
    } else if m[current.1] == 'E' && current.0 == min_cost[&current.1] {
        hashset! {current.1}
    } else {
        let mut tiles = hashset! {};
        let neighbours = neighbours(current.0, current.1, current.2);
        for n in neighbours {
            if m[n.1] == '.' || m[n.1] == 'E' {
                let t = count_tiles(n, min_cost, m);
                if !t.is_empty() {
                    for t in t {
                        tiles.insert(t);
                    }
                    tiles.insert(n.1);
                }
            }
        }
        tiles
    };
    res
}

fn neighbours(
    cost: u64,
    x: (usize, usize),
    d: (isize, isize),
) -> Vec<(u64, (usize, usize), (isize, isize))> {
    let mut n = vec![];
    // continue direction
    n.push((
        cost + 1,
        (
            x.0.checked_add_signed(d.0).unwrap(),
            x.1.checked_add_signed(d.1).unwrap(),
        ),
        d,
    ));
    // turn
    for d in turn(d) {
        n.push((
            cost + 1001,
            (
                x.0.checked_add_signed(d.0).unwrap(),
                x.1.checked_add_signed(d.1).unwrap(),
            ),
            d,
        ));
    }
    n
}

fn print(m: &M, c_m: &I) {
    for y in 0..c_m.len_of(Axis(0)) {
        println!();
        for x in 0..c_m.len_of(Axis(1)) {
            if m[(y, x)] == '.' || m[(y, x)] == 'E' {
                print!(" {:5?}", c_m[(y, x)]);
            } else {
                print!("      ");
            }
        }
    }
    println!();
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
                content: "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
                expected: "45",
            },
            //             Example {
            //                 content: "#################
            // #...#...#...#..E#
            // #.#.#.#.#.#.#.#.#
            // #.#.#.#...#...#.#
            // #.#.#.#.###.#.#.#
            // #...#.#.#.....#.#
            // #.#.#.#.#.#####.#
            // #.#...#.#.#.....#
            // #.#.#####.#.###.#
            // #.#.#.......#...#
            // #.#.###.#####.###
            // #.#.#...#.....#.#
            // #.#.#.#####.###.#
            // #.#.#.........#.#
            // #.#.#.#########.#
            // #S#.............#
            // #################",
            //                 expected: "64",
            //             },
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
