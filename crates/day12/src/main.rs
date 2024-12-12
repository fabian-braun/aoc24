use crate::MovingBorder::{H, V};
use crate::DIR::LEFT;
use anyhow::Context;
use itertools::Itertools;
use maplit::hashset;
use ndarray::Axis;
use std::time::Instant;
use utilities::{B, I, M};
use DIR::{DOWN, RIGHT, UP};

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
    // let content = utilities::get_input(day).await;
    // let start = Instant::now();
    // let solution = run(content);
    // let time_taken = start.elapsed();
    // println!(
    //     "Actual Solution for day {}: \n{:?}\nin time {:?}",
    //     day, solution, time_taken
    // );
}
pub fn padded_char_matrix(raw: String) -> anyhow::Result<M> {
    let y_len = raw.lines().count() + 2;
    let x_len = raw.lines().next().context("char_matrix")?.len() + 2;
    let mut m = M::default((y_len, x_len));
    raw.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            m[(y + 1, x + 1)] = c;
        })
    });
    Ok(m)
}

fn run(input: String) -> anyhow::Result<String> {
    let m = padded_char_matrix(input)?;
    assert!(m.is_square());
    let cnt = m.len_of(Axis(0));
    let mut area = I::default((cnt, cnt));
    let mut known = B::default((cnt, cnt));
    let mut borders = hashset! {};
    m.indexed_iter().for_each(|((y, x), c)| {
        if y == 0 || x == 0 || y == cnt - 1 || x == cnt - 1 {
            return;
        }
        [
            V {
                c: *c,
                l: (y, x - 1),
                r: (y, x),
            },
            V {
                c: *c,
                l: (y, x),
                r: (y, x + 1),
            },
            H {
                c: *c,
                t: (y - 1, x),
                b: (y, x),
            },
            H {
                c: *c,
                t: (y, x),
                b: (y + 1, x),
            },
        ]
        .into_iter()
        .filter(|moving_border: MovingBorder| moving_border.is_border(&m).is_some())
        .for_each(|moving_border: MovingBorder| borders.insert(moving_border));
        let mut explored = vec![];
        explore_region((y, x), *c, &m, &mut explored, &mut known);
        let cnt = explored.len();
        for e in &explored {
            area[e] = cnt;
        }
    });
    let mut borders = borders.into_iter().collect_vec();
    let mut visited = hashset! {};
    loop {
        let border = borders.pop();
        let Some(border) = border else { break };
        if visited.contains(border) {
            continue;
        }
        visited.insert(border);
    }

    let cost = 3;
    Ok(cost.to_string())
}

fn left((y, x): (usize, usize)) -> (usize, usize) {
    (y, x - 1)
}
fn up((y, x): (usize, usize)) -> (usize, usize) {
    (y - 1, x)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum MovingBorder {
    V {
        c: char,
        l: (usize, usize), // left
        r: (usize, usize), // right
    },
    H {
        c: char,
        t: (usize, usize), // top
        b: (usize, usize), // bottom
    },
}
impl MovingBorder {
    fn is_border(self, m: &M) -> Option<Self> {
        let (a, b) = self.adj();
        if m[a] != m[b] {
            Some(self)
        } else {
            None
        }
    }
    fn adj(self) -> ((usize, usize), (usize, usize)) {
        match self {
            V { l, r, .. } => ((l.0, l.1), (r.0, r.1)),
            H { t, b, .. } => ((t.0, t.1), (b.0, b.1)),
        }
    }
    fn traverse(self, m: &M) -> [Self; 2] {
        let mut next = vec![];
        let first = match self {
            V { l, r, c } => {

            },
            H { .. } => {}
        };
        [first, first]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum DIR {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
impl DIR {
    fn go(self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            LEFT => (y, x - 1),
            RIGHT => (y, x + 1),
            UP => (y - 1, x),
            DOWN => (y + 1, x),
        }
    }
    fn turn(self) -> Self {
        match self {
            LEFT => DOWN,
            RIGHT => UP,
            UP => LEFT,
            DOWN => RIGHT,
        }
    }
}
fn explore_region(
    start: (usize, usize),
    r: char,
    m: &M,
    explored: &mut Vec<(usize, usize)>,
    known: &mut B,
) {
    if known[start] || m[start] != r {
        return;
    }
    explored.push(start);
    known[start] = true;
    for n in neighbours(start) {
        explore_region(n, r, m, explored, known);
    }
}

fn neighbours(x: (usize, usize)) -> [(usize, usize); 4] {
    [
        (x.0, x.1 + 1),
        (x.0, x.1 - 1),
        (x.0 + 1, x.1),
        (x.0 - 1, x.1),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
