use itertools::Itertools;
use maplit::hashset;
use ndarray::Axis;
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
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse::<usize>().unwrap() + 1;
            let y: usize = y.parse::<usize>().unwrap() + 1;
            (y, x)
        })
        .collect_vec();
    bytes.iter().take(first_n).for_each(|byte| {
        m[*byte] = '#';
    });
    print(&m);
    let start = (1, 1);
    let end = (dim, dim);
    let result = bytes.iter().skip(first_n).find(|&next_byte| {
        m[*next_byte] = '#';
        !test_path_exists(start, end, &m)
    }).unwrap();
    // invert dimensions and fix padding
    let result = (result.1 - 1, result.0 - 1);

    Ok(format!("{},{}", result.0, result.1))
}

fn test_path_exists(start: (usize, usize), end: (usize, usize), m: &M) -> bool {
    let mut leafs = vec![];
    let mut visited = hashset! {};
    leafs.push(start);
    visited.insert(start);
    while let Some(pos) = leafs.pop() {
        if pos == end {
            return true;
        }
        neighbors(pos).into_iter().for_each(|n_pos| {
            if !visited.contains(&n_pos) && (m[n_pos] != '#') {
                leafs.push(n_pos);
                visited.insert(n_pos);
            }
        })
    }
    false
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
