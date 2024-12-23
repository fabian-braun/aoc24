use itertools::Itertools;
use maplit::hashset;
use std::collections::HashSet;
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
    let mut edges: Vec<(usize, usize)> = vec![];
    input.lines().filter(|l| !l.is_empty()).for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        let mut a_idx = nodes_s.iter().position(|x| x == a);
        if a_idx.is_none() {
            a_idx = Some(nodes_s.len());
            nodes_s.push(a.to_string());
        }
        let mut b_idx = nodes_s.iter().position(|x| x == b);
        if b_idx.is_none() {
            b_idx = Some(nodes_s.len());
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
    }
    let subgraphs = (0..nodes_s.len()).collect_vec();
    let mut subgraphs_2: Vec<Vec<bool>> = vec![];
    for (a, b) in subgraphs.iter().tuple_combinations() {
        if m[(*a, *b)] {
            let mut x = vec![false; nodes_s.len()];
            x[*a] = true;
            x[*b] = true;
            subgraphs_2.push(x);
        }
    }
    let mut subgraphs_prev = subgraphs_2;
    for i in 3..100 {
        dbg!(subgraphs_prev.len());
        let mut subgraphs_next: Vec<Vec<bool>> = vec![];
        for a in subgraphs_prev.iter() {
            for b in 0..nodes_s.len() {
                if a.iter()
                    .enumerate()
                    .filter(|(_ax, is_in)| **is_in)
                    .all(|(ax, _is_in)| m[(ax, b)])
                {
                    let mut merged = a.clone();
                    merged[b] = true;
                    // if !subgraphs_next.iter().contains(&merged) {
                    subgraphs_next.push(merged);
                    // }
                }
            }
        }
        if subgraphs_next.len() == 0 {
            break;
        }
        subgraphs_prev = subgraphs_next;
    }

    let mut result = subgraphs_prev
        .iter()
        .next()
        .unwrap()
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(n_idx, is_in)| {
            if is_in {
                Some(nodes_s[n_idx].clone())
            } else {
                None
            }
        })
        .collect_vec();
    result.sort_unstable();
    Ok(result.into_iter().join(","))
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
