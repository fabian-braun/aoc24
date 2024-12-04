use itertools::Itertools;
use ndarray::{Array2, Axis};

#[tokio::main]
async fn main() {
    let content = utilities::get_input(4).await;
    let m = utilities::char_matrix(content);
    let mut result = 0;
    m.indexed_iter().for_each(|((y, x), &c)| match c {
        'X' => {
            for c in candidates(
                (y as i64, x as i64),
                m.len_of(Axis(0)) as i64,
                m.len_of(Axis(1)) as i64,
            ) {
                if 'M' == m[(c[0].0 as usize, c[0].1 as usize)]
                    && 'A' == m[(c[1].0 as usize, c[1].1 as usize)]
                    && 'S' == m[(c[2].0 as usize, c[2].1 as usize)]
                {
                    result += 1;
                }
            }
        }
        _ => {}
    });

    println!("Part I solution: {}", result);
}

fn candidates((y, x): (i64, i64), y_len: i64, x_len: i64) -> Vec<[(i64, i64); 3]> {
    [
        [(y, x + 1), (y, x + 2), (y, x + 3)],
        [(y + 1, x + 1), (y + 2, x + 2), (y + 3, x + 3)],
        [(y + 1, x), (y + 2, x), (y + 3, x)],
        [(y + 1, x - 1), (y + 2, x - 2), (y + 3, x - 3)],
        [(y, x - 1), (y, x - 2), (y, x - 3)],
        [(y - 1, x - 1), (y - 2, x - 2), (y - 3, x - 3)],
        [(y - 1, x), (y - 2, x), (y - 3, x)],
        [(y - 1, x + 1), (y - 2, x + 2), (y - 3, x + 3)],
    ]
    .into_iter()
    .filter(|coords| {
        coords
            .iter()
            .all(|(y, x)| y < &y_len && x < &x_len && y >= &0 && x >= &0)
    })
    .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
