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
    let result: u64 = input
        .lines()
        .map(|secret_num| {
            let mut secret_num: u64 = secret_num.parse().unwrap();
            for _ in 0..2000 {
                secret_num = calc_next(secret_num);
            }
            secret_num
        })
        .sum();
    Ok(result.to_string())
}

fn calc_next(n: u64) -> u64 {
    let mix = n * 64;
    let n = mix ^ n;
    let n = n % 16777216;

    let mix = n / 32;
    let n = mix ^ n;
    let n = n % 16777216;

    let mix = n * 2048;
    let n = mix ^ n;
    let n = n % 16777216;

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
