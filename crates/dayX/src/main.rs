use ndarray::Axis;
use utilities::char_matrix;

#[tokio::main]
async fn main() {
    let content = utilities::get_example(1).await;

    println!("Solution: {:?}", run(content));
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
