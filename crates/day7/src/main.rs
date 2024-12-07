use itertools::Itertools;

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
    // let content = utilities::get_input(day).await;
    // println!("Actual Solution for day {}: \n{:?}\n", day, run(content));
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
            if dfs(tv, 0_i64, &rhs, 0) {
                println!("✅ {}", line);
                Some(tv)
            } else {
                println!("❌ {}", line);
                None
            }
        })
        .sum();
    Ok(result.to_string())
}

fn dfs(tgt: i64, tv_c: i64, rhs: &[i64], i: usize) -> bool {
    if tv_c > tgt || i >= rhs.len() {
        false
    } else if i == rhs.len() - 2
        && (tv_c + cc(rhs[i], rhs[i + 1]) == tgt || tv_c * cc(rhs[i], rhs[i + 1]) == tgt)
    {
        true
    } else if i == rhs.len() - 1 {
        tv_c + rhs[i] == tgt || tv_c * rhs[i] == tgt
    } else {
        dfs(tgt, tv_c + rhs[i], rhs, i + 1)
            || dfs(tgt, tv_c * rhs[i], rhs, i + 1)
            || dfs(tgt, tv_c + cc(rhs[i], rhs[i + 1]), rhs, i + 2)
            || dfs(tgt, tv_c * cc(rhs[i], rhs[i + 1]), rhs, i + 2)
    }
}

fn cc(a: i64, b: i64) -> i64 {
    let len_b = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10_i64.pow(len_b) + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_something() {
        assert_eq!(12233324324, cc(1223, 3324324))
    }
}
