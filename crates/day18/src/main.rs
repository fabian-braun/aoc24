use itertools::Itertools;
use maplit::hashmap;
use ndarray::Axis;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;
use std::usize;
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
    println!(
        "Example Solution for day {}: \n{:?}\n",
        day,
        run(content, 7, 12)
    );
    let content = utilities::get_input(day).await;
    let start = Instant::now();
    let solution = run(content, 71, 1024);
    let time_taken = start.elapsed();
    println!(
        "Actual Solution for day {}: \n{:?}\nin time {:?}",
        day, solution, time_taken
    );
}

fn run(input: String, dim: usize, first_n: usize) -> anyhow::Result<String> {
    let mut m = M::default((dim + 2, dim + 2));
    m.fill('.');
    for i in 0..dim + 2 {
        m[(0, i)] = '#';
        m[(i, 0)] = '#';
        m[(dim + 1, i)] = '#';
        m[(i, dim + 1)] = '#';
    }
    let bytes = input
        .lines()
        .take(first_n)
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse::<usize>().unwrap() + 1;
            let y: usize = y.parse::<usize>().unwrap() + 1;
            (y, x)
        })
        .collect_vec();
    bytes.iter().for_each(|byte| {
        m[*byte] = '#';
    });
    print(&m);
    let start = (1, 1);
    let end = (dim, dim);
    let mut min_cost = hashmap! { start => 0 };
    let mut pred: HashMap<(usize, usize), (usize, usize)> = hashmap! {};
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    while let Some(Reverse((cost, o_pos))) = heap.pop() {
        if cost > min_cost[&o_pos] {
            continue;
        }
        let new_cost = cost + 1;
        neighbors(o_pos).into_iter().for_each(|n_pos| {
            if new_cost < *min_cost.get(&n_pos).unwrap_or(&usize::MAX) && (m[n_pos] != '#') {
                heap.push(Reverse((new_cost, n_pos)));
                min_cost.insert(n_pos, new_cost);
                pred.insert(n_pos, o_pos);
            }
        })
    }
    let mut path = vec![];
    let mut curr = end;
    while let Some(p) = pred.get(&curr) {
        path.push(*p);
        curr = *p;
    }

    Ok(path.len().to_string())
}

fn neighbors(pos: (usize, usize)) -> [(usize, usize); 4] {
    [
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_examples() {}
}
