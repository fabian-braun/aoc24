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
    println!(
        "Actual Solution for day {}: \n{:?}\nin time {:?}",
        day, solution, time_taken
    );
}

fn run(input: String) -> anyhow::Result<String> {
    let result: usize = input
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.split("\n");
            let (dy1, dx1): (u64, usize) = lines
                .next()
                .map(|line| {
                    let (x, y) = line.split_once(", ").unwrap();
                    let x: usize = x.split_once("+").unwrap().1.parse().unwrap();
                    let y: u64 = y.split_once("+").unwrap().1.parse().unwrap();
                    (y, x)
                })
                .unwrap();
            let (dy2, dx2): (u64, usize) = lines
                .next()
                .map(|line| {
                    let (x, y) = line.split_once(", ").unwrap();
                    let x: usize = x.split_once("+").unwrap().1.parse().unwrap();
                    let y: u64 = y.split_once("+").unwrap().1.parse().unwrap();
                    (y, x)
                })
                .unwrap();

            let (y, x): (u64, usize) = lines
                .next()
                .map(|line| {
                    let (x, y) = line.split_once(", ").unwrap();
                    let x: usize = x.split_once("=").unwrap().1.parse().unwrap();
                    let y: u64 = y.split_once("=").unwrap().1.parse().unwrap();
                    (y, x)
                })
                .unwrap();
            let y = y + 10000000000000;
            let x = x + 10000000000000;
            ((dy1, dx1), (dy2, dx2), (y, x))
        })
        .filter_map(|riddle| solve_riddle(riddle.0, riddle.1, riddle.2))
        .sum();
    Ok(result.to_string())
}

fn solve_riddle(v1: (u64, usize), v2: (u64, usize), t: (u64, usize)) -> Option<usize> {
    solve_eq(v1, v2, t).map(|(a, b)| a * 3 + b)
}

fn solve_eq(
    (dy1, dx1): (u64, usize),
    (dy2, dx2): (u64, usize),
    (ty, tx): (u64, usize),
) -> Option<(usize, usize)> {
    let (a, b) = if dy1 / dy2 == (dx1 / dx2) as u64 && dy2 / dy1 == (dx2 / dx1) as u64 {
        // the first button is 3 times more expensive
        if dy1 / dy2 >= 3 {
            (tx / dx1, 0_usize)
        } else {
            (0_usize, tx / dx2)
        }
    } else {
        let t1 = ty as i64;
        let t2 = tx as i64;
        let v11 = dy1 as i64;
        let v12 = dx1 as i64;
        let v21 = dy2 as i64;
        let v22 = dx2 as i64;
        let b = (t2 * v11 - t1 * v12) / (v22 * v11 - v21 * v12);
        let a = (t1 - b * v21) / v11;
        let b = b.max(0) as usize;
        let a = a.max(0) as usize;
        (a, b)
    };
    if a * dy1 as usize + b * dy2 as usize == ty as usize && a * dx1 + b * dx2 == tx {
        Some((a, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_eq;

    #[test]
    fn test_something() {
        assert_eq!(Some((1, 1)), solve_eq((11, 21), (3, 5), (14, 26)));
        assert_eq!(None, solve_eq((1, 1), (2, 2), (14, 26)));
        assert_eq!(Some((0, 7)), solve_eq((1, 1), (2, 2), (14, 14)));
        assert_eq!(Some((0, 2)), solve_eq((1, 1), (7, 7), (14, 14)));
        assert_eq!(Some((0, 14)), solve_eq((2, 2), (1, 1), (14, 14)));
        assert_eq!(Some((80, 40)), solve_eq((34, 94), (67, 22), (5400, 8400)));
    }
}
