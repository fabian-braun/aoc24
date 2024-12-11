use itertools::Itertools;
use maplit::hashmap;
use std::collections::HashMap;
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
    println!("Actual Solution for day {}: \n{:?}\nin time {:?}", day, solution, time_taken);
}

fn run(input: String) -> anyhow::Result<String> {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let mut total_stones = 0;
    let mut h: HashMap<(u64, usize), usize> = hashmap! {};
    stones.into_iter().for_each(|num| {
        total_stones += compute_count(num, 75, &mut h);
        dbg!(&h.len());
    });
    Ok(total_stones.to_string())
}

fn compute_count(num: u64, remaining_blinks: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    if memo.contains_key(&(num, remaining_blinks)) {
        return memo[&(num, remaining_blinks)];
    }
    if remaining_blinks == 0 {
        return 1;
    }
    let remaining_blinks = remaining_blinks.checked_sub(1).unwrap_or(0);
    let result = if num == 0 {
        compute_count(1, remaining_blinks, memo)
    } else if let Some(parts) = parts(num) {
        compute_count(parts.0, remaining_blinks, memo) + compute_count(parts.1, remaining_blinks, memo)
    } else {
        compute_count(num * 2024, remaining_blinks, memo)
    };
    memo.insert((num, remaining_blinks + 1), result);
    result
}

fn parts(mut num: u64) -> Option<(u64, u64)> {
    let mut digits = 0_u64;
    let mut tmp_num = num;
    while tmp_num > 0 {
        tmp_num /= 10;
        digits += 1;
    }
    if digits % 2 == 1 {
        return None;
    }
    let digits = digits / 2;

    let mut a = num;
    let mut b = 0_u64;
    let mut exp = 1_u64;
    for _ in 0..digits {
        let next_digit = num % 10;
        a = a / 10;
        b += next_digit * exp;
        exp = exp * 10;
        num = num / 10;
    }
    Some((a, b))
}

#[cfg(test)]
mod tests {
    use crate::parts;

    #[test]
    fn test_something() {
        assert_eq!(None, parts(1));
        assert_eq!(Some((1, 2)), parts(12));
        assert_eq!(None, parts(123));
        assert_eq!(Some((12, 34)), parts(1234));
        assert_eq!(None, parts(12345));
        assert_eq!(Some((123, 456)), parts(123456));
        assert_eq!(None, parts(1234567));
        assert_eq!(Some((1234, 5678)), parts(12345678));
    }
}
