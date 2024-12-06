use ndarray::Axis;
use utilities::M;

#[tokio::main]
async fn main() {
    let content = utilities::get_input(4).await;
    println!("Part II solution: {}", run(content).unwrap());
}

fn run(input: String) -> anyhow::Result<String> {
    let m = utilities::char_matrix(input)?;
    let mut result = 0;
    let y_len = m.len_of(Axis(0)) as i64;
    let x_len = m.len_of(Axis(1)) as i64;
    m.indexed_iter().for_each(|((y, x), &ch)| match ch {
        'A' => {
            if let Some(c) = corners((y as i64, x as i64), y_len, x_len) {
                if 'M' == m[(c[0].0, c[0].1)] && 'S' == m[(c[1].0, c[1].1)]
                    || 'S' == m[(c[0].0, c[0].1)] && 'M' == m[(c[1].0, c[1].1)]
                {
                    if 'M' == m[(c[2].0, c[2].1)] && 'S' == m[(c[3].0, c[3].1)]
                        || 'S' == m[(c[2].0, c[2].1)] && 'M' == m[(c[3].0, c[3].1)]
                    {
                        result += 1;
                    }
                }
            }
        }
        _ => {}
    });
    Ok(result.to_string())
}

fn corners((y, x): (i64, i64), y_len: i64, x_len: i64) -> Option<Vec<(usize, usize)>> {
    let corners = [
        (y + 1, x + 1),
        (y - 1, x - 1),
        (y - 1, x + 1),
        (y + 1, x - 1),
    ];
    if corners
        .iter()
        .all(|(y, x)| y < &y_len && x < &x_len && y >= &0 && x >= &0)
    {
        Some(
            corners
                .into_iter()
                .map(|(y, x)| (y as usize, x as usize))
                .collect(),
        )
    } else {
        None
    }
}

const P: [char; 4] = ['X', 'M', 'A', 'S'];

#[allow(dead_code)]
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
