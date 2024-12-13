use anyhow::Context;
use ndarray::Axis;
use std::time::Instant;
use utilities::{B, I, M};

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
pub fn padded_char_matrix(raw: String) -> anyhow::Result<M> {
    let y_len = raw.lines().count() + 2;
    let x_len = raw.lines().next().context("char_matrix")?.len() + 2;
    let mut m = M::default((y_len, x_len));
    raw.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            m[(y + 1, x + 1)] = c;
        })
    });
    Ok(m)
}

fn run(input: String) -> anyhow::Result<String> {
    let m = padded_char_matrix(input)?;
    assert!(m.is_square());
    let cnt = m.len_of(Axis(0));
    let mut area = I::default((cnt, cnt));
    let mut known = B::default((cnt, cnt));
    m.indexed_iter().for_each(|((y, x), c)| {
        if y == 0 || x == 0 || y == cnt - 1 || x == cnt - 1 {
            return;
        }
        let mut explored = vec![];
        explore_region((y, x), *c, &m, &mut explored, &mut known);
        let cnt = explored.len();
        for e in explored {
            area[e] = cnt;
        }
    });

    let mut cost = 0;

    // count corners: convex & concave
    m.indexed_iter().for_each(|((y, x), _c)| {
        if y == cnt - 1 || x == cnt - 1 {
            return;
        }
        let region = [(y, x), (y, x + 1), (y + 1, x + 1), (y + 1, x)]; // clockwise ordering from top-left
        for r_idx in 0..4 {
            if m[region[r_idx]] != m[region[(r_idx + 1) % 4]]
                && m[region[r_idx]] != m[region[(r_idx + 3) % 4]]
            {
                // found a convex corner at (y,x)
                cost += area[region[r_idx]]
            } else if m[region[r_idx]] == m[region[(r_idx + 1) % 4]]
                && m[region[r_idx]] == m[region[(r_idx + 3) % 4]]
                && m[region[r_idx]] != m[region[(r_idx + 2) % 4]]
            {
                // found a concave corner at (y,x)
                cost += area[region[r_idx]]
            }
        }
    });

    Ok(cost.to_string())
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
    for n in neighbours(start) {
        explore_region(n, r, m, explored, known);
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
