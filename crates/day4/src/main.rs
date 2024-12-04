use itertools::{iproduct, Itertools};
use ndarray::Axis;
use utilities::M;

#[tokio::main]
async fn main() {
    let content = utilities::get_input(4).await;
    let m = utilities::char_matrix(content);
    let mut result = 0;
    let y_len = m.len_of(Axis(0)) as i64;
    let x_len = m.len_of(Axis(1)) as i64;
    m.indexed_iter().for_each(|((y, x), &c)| match c {
        'X' => {
            for c in candidates((y as i64, x as i64), y_len, x_len) {
                if 'M' == m[(c[0].0, c[0].1)]
                    && 'A' == m[(c[1].0, c[1].1)]
                    && 'S' == m[(c[2].0, c[2].1)]
                {
                    result += 1;
                }
            }
        }
        _ => {}
    });

    println!("Part I solution: {}", result);

    let mut result = 0;
    m.indexed_iter()
        .for_each(|((y, x), _)| result += dfs(&m, (y as i64, x as i64), y_len, x_len, 0));

    println!("Part II solution: {}", result);
}

fn candidates((y, x): (i64, i64), y_len: i64, x_len: i64) -> Vec<Vec<(usize, usize)>> {
    let offsets = [[1, 2, 3], [-1, -2, -3], [0, 0, 0]];
    let mut candidates = vec![];
    for (offset_y, offset_x) in iproduct!(offsets.clone(), offsets) {
        if !(offset_y[0] == 0 && offset_x[0] == 0) {
            candidates.push([
                (y + offset_y[0], x + offset_x[0]),
                (y + offset_y[1], x + offset_x[1]),
                (y + offset_y[2], x + offset_x[2]),
            ]);
        }
    }
    println!("candidates {}", candidates.len());
    candidates
        .into_iter()
        .filter(|coords| {
            coords
                .iter()
                .all(|(y, x)| y < &y_len && x < &x_len && y >= &0 && x >= &0)
        })
        .map(|coords| {
            coords
                .into_iter()
                .map(|(y, x)| (y as usize, x as usize))
                .collect_vec()
        })
        .collect_vec()
}

const P: [char; 4] = ['X', 'M', 'A', 'S'];

fn dfs(m: &M, (y, x): (i64, i64), y_len: i64, x_len: i64, idx: usize) -> usize {
    if y < 0 || y >= y_len || x < 0 || x >= x_len || m[(y as usize, x as usize)] != P[idx] {
        0
    } else if idx == 3 {
        1
    } else {
        let mut partial_sum = 0;
        for y_sub in y - 1..=y + 1 {
            for x_sub in x - 1..=x + 1 {
                partial_sum += dfs(m, (y_sub, x_sub), y_len, x_len, idx + 1);
            }
        }
        partial_sum
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
