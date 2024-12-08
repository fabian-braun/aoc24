use itertools::Itertools;
use maplit::hashset;
use ndarray::Axis;
use std::collections::HashMap;
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
    let mut unique_locations = hashset! {};
    antenna_groups.into_iter().for_each(|(_c, coords)| {
        for ((y1, x1), (y2, x2)) in coords.iter().tuple_combinations() {
            let dy: i64 = *y1 as i64 - *y2 as i64;
            let dx: i64 = *x1 as i64 - *x2 as i64;
            let y3: i64 = *y1 as i64 + dy;
            let x3: i64 = *x1 as i64 + dx;
            let y4: i64 = *y2 as i64 - dy;
            let x4: i64 = *x2 as i64 - dx;
            if y3 >= 0 && y3 < y_len && x3 >= 0 && x3 < x_len {
                unique_locations.insert((y3, x3));
            }
            if y4 >= 0 && y4 < y_len && x4 >= 0 && x4 < x_len {
                unique_locations.insert((y4, x4));
            }
        }
    });
    Ok(format!("{:?}", unique_locations.len()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
