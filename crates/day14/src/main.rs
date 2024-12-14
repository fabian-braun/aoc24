use itertools::Itertools;
use std::time::Instant;

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
    let start = Instant::now();
    let solution = run(content);
    let time_taken = start.elapsed();
    println!("Actual Solution for day {}: \n{:?}\nin time {:?}", day, solution, time_taken);
}

// X,Y
fn run(input: String) -> anyhow::Result<String> {
    let dim: (usize, u64) = (101, 103);
    let half_idx = (dim.0 / 2, dim.1 / 2);
    let robots = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let mut l = line.split_ascii_whitespace();
                let pos = l.next().unwrap().strip_prefix("p=").unwrap();
                let d = l.next().unwrap().strip_prefix("v=").unwrap();
                let (x, y) = pos.split_once(',').unwrap();
                let (dx, dy) = d.split_once(',').unwrap();
                let x: usize = x.parse().unwrap();
                let y: u64 = y.parse().unwrap();
                let dx: isize = dx.parse().unwrap();
                let dy: i64 = dy.parse().unwrap();
                Some(((x, y), (dx, dy)))
            }
        })
        .collect_vec();
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;
    robots.into_iter().filter_map(|robot| {
        let pos = move_robot(robot, dim, 100);
        (pos.0 != half_idx.0 && pos.1 != half_idx.1).then_some(pos)
    }).for_each(|pos|{
        if pos.0 < half_idx.0 {
            // left
            if pos.1 < half_idx.1 {
                // top
                tl += 1;
            } else {
                // bottom
                bl += 1;
            }
        } else {
            // right
            if pos.1 < half_idx.1 {
                // top
                tr += 1;
            } else {
                // bottom
                br += 1;
            }
        }
    });
    let result: usize = tl * tr * bl * br;
    Ok(result.to_string())
}

fn move_robot(
    robot: ((usize, u64), (isize, i64)),
    dim: (usize, u64),
    mut times: usize,
) -> (usize, u64) {
    let mut pos = robot.0;
    let d = robot.1;
    let mut dd = dim;
    while (d.0.abs() as usize) >= dd.0 {
        dd.0 += dim.0;
    };
    while (d.1.abs() as u64) >= dd.1 {
        dd.1 += dim.1;
    };
    assert!((d.0.abs() as usize) < dd.0);
    assert!((d.1.abs() as u64) < dd.1);
    while times > 0 {
        pos.0 += dd.0;
        pos.0 = (pos.0 as isize + d.0) as usize;
        pos.0 %= dim.0;
        pos.1 += dd.1;
        pos.1 = (pos.1 as i64 + d.1) as u64;
        pos.1 %= dim.1;
        times -= 1;
    }
    pos
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
