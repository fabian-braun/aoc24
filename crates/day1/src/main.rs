use maplit::hashmap;

#[tokio::main]
async fn main() {
    let content = utilities::get_input(1).await;
    let mut col1 = vec![0i64];
    let mut col2 = vec![0i64];
    content.lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();
        col1.push(iter.next().unwrap().parse().unwrap());
        col2.push(iter.next().unwrap().parse().unwrap());
    });
    col1.sort();
    col2.sort();
    let mut dist = 0;
    for i in 0..col1.len() {
        dist += (col1[i] - col2[i]).abs();
    }

    println!("Part I solution: {}", dist);

    let mut counts = hashmap! {};
    content.lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();
        _ = iter.next().unwrap();
        let second: i64 = iter.next().unwrap().parse().unwrap();
        *counts.entry(second).or_insert(0) += 1;
    });
    let mut similarity = 0;
    content.lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();
        let first: i64 = iter.next().unwrap().parse().unwrap();
        let multiplier = counts.get(&first).unwrap_or(&0i64);
        similarity += first * multiplier;
    });
    println!("Part II solution: {}", similarity);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
