use anyhow::Context;
use ndarray::{Array2, Axis};

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
    println!("Actual Solution for day {}: \n{:?}\n", day, run(content));
}

fn run(input: String) -> anyhow::Result<String> {
    let (m, b) = uint_matrix(input)?;
    let mut b = b;
    let starts: Vec<(usize, usize)> = m
        .indexed_iter()
        .filter_map(|((y, x), &c)| if c == 0 { Some((y, x)) } else { None })
        .collect();
    let result: usize = starts
        .iter()
        .map(|(y, x)| dfs(0, (*y, *x), &m, &mut b))
        .sum();
    Ok(result.to_string())
}

fn dfs(tgt: u8, pos: (usize, usize), m: &M, b: &mut B) -> usize {
    let result = if m[pos] == tgt {
        if tgt == 9 && !b[pos] {
            1
        } else {
            dfs(
                tgt + 1,
                ((pos.0 + 1).min(m.len_of(Axis(0)) - 1), pos.1),
                &m,
                b,
            ) + dfs(
                tgt + 1,
                (pos.0.checked_sub(1).unwrap_or(pos.0), pos.1),
                &m,
                b,
            ) + dfs(
                tgt + 1,
                (pos.0, (pos.1 + 1).min(m.len_of(Axis(1)) - 1)),
                &m,
                b,
            ) + dfs(
                tgt + 1,
                (pos.0, pos.1.checked_sub(1).unwrap_or(pos.1)),
                &m,
                b,
            )
        }
    } else {
        0
    };
    b[pos] = true;
    result
}

pub type M = Array2<u8>;
pub type B = Array2<bool>;
pub fn uint_matrix(raw: String) -> anyhow::Result<(M, B)> {
    let y_len = raw.lines().count();
    let x_len = raw.lines().next().context("char_matrix")?.len();
    let mut m = M::default((y_len, x_len));
    let mut b = B::default((y_len, x_len));
    raw.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let i = c.to_string().parse().unwrap_or(u8::MAX);
            m[(y, x)] = i;
        })
    });
    Ok((m, b))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
