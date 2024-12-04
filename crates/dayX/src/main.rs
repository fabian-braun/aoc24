use ndarray::Axis;
use utilities::char_matrix;

#[tokio::main]
async fn main() {
    let content = utilities::get_example(1).await;
    let matrix = char_matrix(content);
    let result: usize = matrix.len_of(Axis(0)) * matrix.len_of(Axis(1));

    println!("Solution: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
