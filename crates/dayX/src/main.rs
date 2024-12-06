use ndarray::Axis;
use utilities::char_matrix;

const VERSION: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    let day = VERSION
        .strip_prefix("day")
        .unwrap_or_default()
        .parse()
        .unwrap_or(1);
    let content = utilities::get_example(day).await;

    println!("Solution for day {}: {:?}", day, run(content));
}

fn run(input: String) -> anyhow::Result<String> {
    let matrix = char_matrix(input)?;
    let result: usize = matrix.len_of(Axis(0)) * matrix.len_of(Axis(1));
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
