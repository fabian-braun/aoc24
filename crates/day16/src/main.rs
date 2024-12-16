use maplit::hashmap;
use ndarray::Axis;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;
use utilities::char_matrix;

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
    let start: (u64, (usize, usize), (isize, isize)) = (0, (y_len - 2, 1), EAST);
    let end = (1, x_len - 2);
    println!("search path from {start:?} to {end:?}");
    let mut min_cost = hashmap! { start.1 => start.0 };
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(start));
    while let Some(Reverse((cost, pos, d))) = heap.pop() {
        if pos == end {
            break;
        }
        if cost > min_cost[&pos] {
            continue;
        }
        neighbours(cost, pos, d).into_iter().for_each(|n| {
            if n.0 < *min_cost.get(&n.1).unwrap_or(&u64::MAX) && (m[n.1] == '.' || m[n.1] == 'E' ) {
                heap.push(Reverse(n));
                min_cost.insert(n.1, n.0);
            }
        })
    }
    Ok(min_cost[&end].to_string())
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
                expected: "7036",
            },
            Example {
                content: "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
                expected: "11048",
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
