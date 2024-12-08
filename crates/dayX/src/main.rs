use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use utilities::char_matrix;

const VERSION: &str = env!("CARGO_PKG_NAME");
pub type B = Array2<bool>;

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
    println!("Actual Solution for day {}: \n{:?}\n", day, run(content));
}

fn run(input: String) -> anyhow::Result<String> {
    let matrix = char_matrix(input)?;
    let y_len = matrix.len_of(Axis(0)) as i64;
    let x_len = matrix.len_of(Axis(1)) as i64;
    let antenna_groups: HashMap<char, Vec<(usize, usize)>> = matrix
        .indexed_iter()
        .filter_map(|((y, x), &c)| match c {
            '.' => None,
            _ => Some((c, (y, x))),
        })
        .into_group_map();
    let mut unique_locations = B::default((y_len as usize, x_len as usize));
    antenna_groups.into_iter().for_each(|(_c, coords)| {
        for ((y1, x1), (y2, x2)) in coords.iter().tuple_combinations() {
            unique_locations[(*y1, *x1)] = true;
            unique_locations[(*y2, *x2)] = true;
            let y1 = *y1 as i64;
            let x1 = *x1 as i64;
            let y2 = *y2 as i64;
            let x2 = *x2 as i64;
            let dy: i64 = y1 - y2;
            let dx: i64 = x1 - x2;
            let mut y3: i64 = y1 + dy;
            let mut x3: i64 = x1 + dx;
            let mut y4: i64 = y2 - dy;
            let mut x4: i64 = x2 - dx;
            while y3 >= 0 && y3 < y_len && x3 >= 0 && x3 < x_len {
                unique_locations[(y3 as usize, x3 as usize)] = true;
                y3 = y3 + dy;
                x3 = x3 + dx;
            }
            while y4 >= 0 && y4 < y_len && x4 >= 0 && x4 < x_len {
                unique_locations[(y4 as usize, x4 as usize)] = true;
                y4 = y4 - dy;
                x4 = x4 - dx;
            }
        }
    });
    Ok(format!(
        "{:?}",
        unique_locations.iter().filter(|x| **x).count()
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
