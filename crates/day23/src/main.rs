use itertools::Itertools;
use std::time::Instant;
use utilities::B;

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
    let solution = run(content).unwrap();
    assert_ne!("11011".to_string(), solution.to_string());
    assert_ne!("2351".to_string(), solution.to_string());
    let time_taken = start.elapsed();
    println!(
        "Actual Solution for day {}: \n{:?}\nin time {:?}",
        day, solution, time_taken
    );
}

fn run(input: String) -> anyhow::Result<String> {
    let mut nodes_s: Vec<String> = vec![];
    let mut nodes: Vec<usize> = vec![];
    let mut edges: Vec<(usize, usize)> = vec![];
    input.lines().filter(|l| !l.is_empty()).for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        let mut a_idx = nodes_s.iter().position(|x| x == a);
        if a_idx.is_none() {
            a_idx = Some(nodes_s.len());
            nodes.push(a_idx.unwrap());
            nodes_s.push(a.to_string());
        }
        let mut b_idx = nodes_s.iter().position(|x| x == b);
        if b_idx.is_none() {
            b_idx = Some(nodes_s.len());
            nodes.push(b_idx.unwrap());
            nodes_s.push(b.to_string());
        }
        if !edges.contains(&(a_idx.unwrap(), b_idx.unwrap())) {
            edges.push((a_idx.unwrap(), b_idx.unwrap()));
        }
    });
    let mut m = B::default((nodes_s.len(), nodes_s.len()));
    for (a, b) in &edges {
        m[(*a, *b)] = true;
        m[(*b, *a)] = true;
        m[(*a, *a)] = true;
        m[(*b, *b)] = true;
    }
    let subgraphs = nodes.clone();
    let mut subgraphs_2 = vec![];
    for (a, b) in subgraphs.iter().tuple_combinations() {
        if m[(*a, *b)] {
            subgraphs_2.push(vec![a, b]);
        }
    }
    let mut subgraphs_3 = vec![];
    for (a, b) in subgraphs_2.iter().tuple_combinations() {
        let mut merged = a.iter().chain(b.iter()).unique().collect_vec();
        if merged.len() == 3 && a.iter().all(|&a| b.iter().all(|&b| m[(*a, *b)])) {
            merged.sort_unstable();
            if !subgraphs_3.iter().contains(&merged) {
                subgraphs_3.push(merged);
            }
        }
    }
    let result = subgraphs_3
        .iter()
        .filter(|sg| sg.iter().any(|n_idx| nodes_s[***n_idx].starts_with('t')))
        .count();
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
