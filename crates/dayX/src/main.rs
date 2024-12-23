use std::time::Instant;
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
    let start = Instant::now();
    let solution = run(content);
    let time_taken = start.elapsed();
    println!("Actual Solution for day {}: \n{:?}\nin time {:?}", day, solution, time_taken);
}

fn run(input: String) -> anyhow::Result<String> {
    let matrix = char_matrix(input)?;
    let result: usize = matrix.len_of(Axis(0)) * matrix.len_of(Axis(1));
    Ok(result.to_string())
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
