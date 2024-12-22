use itertools::Itertools;
use maplit::hashmap;
use std::collections::{HashMap, VecDeque};
use std::iter::repeat_n;
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
    let start_nums: Vec<i64> = input
        .lines()
        .map(|secret_num| secret_num.parse().unwrap())
        .collect_vec();
    let seq_to_bananas = start_nums
        .iter()
        .map(|secret_num| {
            let mut seq_to_bananas: HashMap<(i64, i64, i64, i64), i64> = hashmap! {};
            let mut secret_num_a: i64 = *secret_num;
            let mut seq = VecDeque::from([0_i64, 0_i64, 0_i64, 0_i64]);
            for _ in 0..3 {
                let secret_num_b = calc_next(secret_num_a);
                let price_a = secret_num_a % 10;
                let price_b = secret_num_b % 10;
                secret_num_a = secret_num_b;
                _ = seq.pop_front();
                seq.push_back(price_b - price_a);
            }
            for _ in 3..2000 {
                let secret_num_b = calc_next(secret_num_a);
                let price_a = secret_num_a % 10;
                let price_b = secret_num_b % 10;
                secret_num_a = secret_num_b;
                _ = seq.pop_front();
                seq.push_back(price_b - price_a);
                if seq_to_bananas.contains_key(&(seq[0], seq[1], seq[2], seq[3])) {
                    continue;
                }
                seq_to_bananas.insert((seq[0], seq[1], seq[2], seq[3]), price_b);
            }
            seq_to_bananas
        })
        .collect_vec();
    let mut max_bananas = 0_i64;
    repeat_n(-9_i64..9_i64, 4)
        .multi_cartesian_product()
        .for_each(|v| {
            let mut total_bananas = 0;
            for seq_to_bananas in &seq_to_bananas {
                let bananas = seq_to_bananas.get(&(v[0], v[1], v[2], v[3])).unwrap_or(&0);
                total_bananas += bananas;
            }
            max_bananas = max_bananas.max(total_bananas);
        });
    Ok(max_bananas.to_string())
}

fn calc_next(n: i64) -> i64 {
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
    fn test_examples() {}
}
