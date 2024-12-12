use std::time::Instant;
use ndarray::Axis;
use utilities::{char_matrix, B, I, M};

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
    let m = char_matrix(input)?;
    assert!(m.is_square());
    let cnt = m.len_of(Axis(0));
    let mut area = I::default((cnt, cnt));
    let mut known = B::default((cnt, cnt));
    m.indexed_iter().for_each(|((y, x), c)| {
        let mut explored = vec![];
        explore_region((y, x), *c, &m, &mut explored, &mut known);
        let cnt = explored.len();
        for e in explored {
            area[e] = cnt;
        }
    });
    let mut total_fence = 0;
    for i0 in 0..cnt {
        for j1 in 1..cnt {
            if m[(i0, j1 - 1)] != m[(i0, j1)] {
                total_fence += area[(i0, j1 - 1)];
                total_fence += area[(i0, j1)];
            }
            if m[(j1 - 1, i0)] != m[(j1, i0)] {
                total_fence += area[(j1 - 1, i0)];
                total_fence += area[(j1, i0)];
            }
        }
        total_fence += area[(0, i0)];
        total_fence += area[(i0, 0)];
        total_fence += area[(cnt - 1, i0)];
        total_fence += area[(i0, cnt - 1)];
    }

    Ok(total_fence.to_string())
}

fn explore_region(
    start: (usize, usize),
    r: char,
    m: &M,
    explored: &mut Vec<(usize, usize)>,
    known: &mut B,
) {
    if known[start] || m[start] != r {
        return;
    }
    explored.push(start);
    known[start] = true;
    for n in neighbours(start, m) {
        explore_region(n, r, m, explored, known);
    }
}

fn neighbours(x: (usize, usize), m: &M) -> [(usize, usize); 4] {
    [
        (x.0, (x.1 + 1).min(m.len_of(Axis(1)) - 1)),
        (x.0, x.1.checked_sub(1).unwrap_or(0)),
        ((x.0 + 1).min(m.len_of(Axis(0)) - 1), x.1),
        (x.0.checked_sub(1).unwrap_or(0), x.1),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
