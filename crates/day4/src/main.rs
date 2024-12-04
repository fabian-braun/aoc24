use itertools::Itertools;
use ndarray::Array2;

type M = Array2<char>;

#[tokio::main]
async fn main() {
    let content = utilities::get_example(4).await;
    let y_len = content.lines().count();
    let x_len = content.lines().next().unwrap().len();
    let mut m = M::default((y_len, x_len));
    let mut x_coordinates = vec![];
    content.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            'X' => {
                m[(y, x)] = c;
                x_coordinates.push((y as i64, x as i64));
            }
            'M' | 'A' | 'S' => m[(y, x)] = c,
            _ => m[(y, x)] = ' ',
        })
    });
    let mut result = 0;
    for x in x_coordinates {
        for c in candidates(x, y_len as i64, x_len as i64) {
            if 'M' == m[(c[0].0 as usize, c[0].1 as usize)]
                && 'A' == m[(c[1].0 as usize, c[1].1 as usize)]
                && 'S' == m[(c[2].0 as usize, c[2].1 as usize)]
            {
                result += 1;
            }
        }
    }

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
