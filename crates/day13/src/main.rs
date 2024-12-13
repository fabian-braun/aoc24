use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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
    let result: usize = input
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.split("\n");
            let (dy1, dx1): (u64, usize) = lines
                .next()
                .map(|line| {
                    let (x, y) = line.split_once(", ").unwrap();
                    let x: usize = x.split_once("+").unwrap().1.parse().unwrap();
                    let y: u64 = y.split_once("+").unwrap().1.parse().unwrap();
                    (y, x)
                })
                .unwrap();
            let (dy2, dx2): (u64, usize) = lines
                .next()
                .map(|line| {
                    let (x, y) = line.split_once(", ").unwrap();
                    let x: usize = x.split_once("+").unwrap().1.parse().unwrap();
                    let y: u64 = y.split_once("+").unwrap().1.parse().unwrap();
                    (y, x)
                })
                .unwrap();

            let (y, x): (u64, usize) = lines
                .next()
                .map(|line| {
                    let (x, y) = line.split_once(", ").unwrap();
                    let x: usize = x.split_once("=").unwrap().1.parse().unwrap();
                    let y: u64 = y.split_once("=").unwrap().1.parse().unwrap();
                    (y, x)
                })
                .unwrap();
            ((dy1, dx1), (dy2, dx2), (y, x))
        })
        .filter_map(|riddle| solve_riddle(riddle))
        .sum();
    Ok(result.to_string())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (u64, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_riddle(
    ((dy1, dx1), (dy2, dx2), (ty, tx)): ((u64, usize), (u64, usize), (u64, usize)),
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut min_costs = HashMap::new();
    let start = State {
        cost: 0,
        position: (0, 0),
    };
    min_costs.insert(start.position, start.cost);
    heap.push(start);
    while let Some(State { cost, position }) = heap.pop() {
        if position == (ty, tx) {
            return Some(cost);
        }
        if position.0 > ty || position.1 > tx {
            continue;
        }
        if cost > min_costs[&position] {
            continue;
        }
        let next = State {
            cost: cost + 3,
            position: (position.0 + dy1, position.1 + dx1),
        };
        if next.cost < *min_costs.get(&next.position).unwrap_or(&usize::MAX) {
            heap.push(next);
            min_costs.insert(next.position, next.cost);
        }
        let next = State {
            cost: cost + 1,
            position: (position.0 + dy2, position.1 + dx2),
        };
        if next.cost < *min_costs.get(&next.position).unwrap_or(&usize::MAX) {
            heap.push(next);
            min_costs.insert(next.position, next.cost);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
