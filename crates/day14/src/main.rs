use average::Variance;
use itertools::Itertools;
use ndarray::Axis;
use std::time::Instant;
use utilities::{B, I, M};

const VERSION: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    let day = VERSION
        .strip_prefix("day")
        .unwrap_or_default()
        .parse()
        .unwrap_or(1);
    let content = utilities::get_example(day).await;
    // println!("Example Solution for day {}: \n{:?}\n", day, run(content));
    let content = utilities::get_input(day).await;
    let start = Instant::now();
    let solution = run(content);
    let time_taken = start.elapsed();
    println!(
        "Actual Solution for day {}: \n{:?}\nin time {:?}",
        day, solution, time_taken
    );
}

// X,Y
fn run(input: String) -> anyhow::Result<String> {
    let dim: (usize, u64) = (101, 103);
    let mut robots = input
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
    let mut seconds = 0;
    let mut min_variance = f64::MAX;
    let mut max_seconds = 0;
    while seconds < 10001 {
        let variance = robot_position_variance(&robots);
        if variance < min_variance {
            min_variance = variance;
            max_seconds = seconds;
            println!("####################################################################################     {}", seconds);
            print_christmas_tree(&robots, dim);
        }
        robots = robots
            .into_iter()
            .map(|mut robot| {
                let pos = move_robot(robot, dim, 1);
                robot.0 = pos;
                robot
            })
            .collect_vec();
        seconds += 1;
    }
    Ok(max_seconds.to_string())
}
// wrong 15, 25, 66

fn robot_position_variance(robots: &[((usize, u64), (isize, i64))]) -> f64 {
    let x_variance: Variance = robots.iter().map(|x| x.0 .0 as f64).collect();
    let y_variance: Variance = robots.iter().map(|x| x.0 .1 as f64).collect();

    x_variance.sample_variance() + y_variance.sample_variance()
}

fn print_christmas_tree(robots: &[((usize, u64), (isize, i64))], dim: (usize, u64)) {
    let mut m = M::default((dim.0, dim.1 as usize));
    m.fill(' ');
    for r in robots {
        m[(r.0 .0, r.0 .1 as usize)] = 'X';
    }
    for y in 0..m.len_of(Axis(1)) {
        println!();
        for x in 0..m.len_of(Axis(0)) {
            print!("{}", m[(x, y)]);
        }
    }
    println!();
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
    }
    while (d.1.abs() as u64) >= dd.1 {
        dd.1 += dim.1;
    }
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
}
