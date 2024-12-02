use itertools::Itertools;

#[tokio::main]
async fn main() {
    let content = utilities::get_input(2).await;
    let result: usize = content
        .lines()
        .filter(|line| {
            let levels = line
                .split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect_vec();
            return if is_levels_safe(&levels, None) {
                true
            } else {
                (0..levels.len()).any(|idx| is_levels_safe(&levels, Some(idx)))
            }
        })
        .count();

    println!("Part I solution: {}", result);
}

fn is_levels_safe(input: &[i64], skip: Option<usize>) -> bool {
    let mut prev: Option<i64> = None;
    let mut asc: Option<bool> = None;
    for (idx, l) in input.iter().enumerate() {
        if let Some(skip) = skip {
            if skip == idx {
                continue;
            }
        }
        if let Some(prev) = prev {
            if 0 == (l - prev).abs() || (l - prev).abs() > 3 {
                return false;
            } else if let Some(asc) = asc {
                if (l > &prev) != asc {
                    return false;
                }
            } else {
                asc = Some(l > &prev);
            }
        }
        prev = Some(*l)
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
