use itertools::Itertools;
use ndarray::Axis;
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

fn run(input: String) -> anyhow::Result<String> {
    let mut iter = input.lines();
    let towels = iter.next().unwrap().split(", ").collect_vec();
    let designs = iter.filter(|line| !line.is_empty()).collect_vec();

    let result = designs.iter().filter(|design| dfs(&towels, design)).count();
    Ok(result.to_string())
}

fn dfs(towels: &[&str], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) {
            if dfs(&towels, &design[towel.len()..]) {
                return true;
            }
        }
    }
    false
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
