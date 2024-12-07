use itertools::Itertools;
use ndarray::Axis;
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
    println!("Actual Solution for day {}: \n{:?}\n", day, run(content));
}

fn run(input: String) -> anyhow::Result<String> {
    let result: i64 = input
        .lines()
        .filter_map(|line| {
            let mut line_parts = line.split(": ");
            let tv: i64 = line_parts.next()?.parse().ok()?;
            let rhs: Vec<i64> = line_parts
                .next()?
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect_vec();
            if dfs(tv, rhs[0], &rhs, 1) {
                Some(tv)
            } else {
                None
            }
        })
        .sum();
    Ok(result.to_string())
}

fn dfs(tv_target: i64, tv_current: i64, rhs: &[i64], idx: usize) -> bool {
    if tv_current > tv_target {
        false
    } else if idx == rhs.len() - 1 {
        tv_current + rhs[idx] == tv_target || tv_current * rhs[idx] == tv_target
    } else {
        dfs(tv_target, tv_current + rhs[idx], rhs, idx + 1)
            || dfs(tv_target, tv_current * rhs[idx], rhs, idx + 1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
