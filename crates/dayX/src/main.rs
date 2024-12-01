#[tokio::main]
async fn main() {
    let content = utilities::get_example(1).await;
    let result: usize = content.lines().map(|_line| {}).count();

    println!("Part I solution: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
