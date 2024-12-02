#[tokio::main]
async fn main() {
    let content = utilities::get_input(2).await;
    let result: usize = content
        .lines()
        .filter(|line| {
            let mut prev: Option<i64> = None;
            let mut asc: Option<bool> = None;
            for c in line.split_ascii_whitespace() {
                let l: i64 = c.parse().unwrap();
                if let Some(prev) = prev {
                    if 0 == (l - prev).abs() || (l - prev).abs() > 3 {
                        return false;
                    } else if let Some(asc) = asc {
                        if (l > prev) != asc {
                            return false;
                        }
                    } else {
                        asc = Some(l > prev);
                    }
                }
                prev = Some(l)
            }
            true
        })
        .count();

    println!("Part I solution: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
