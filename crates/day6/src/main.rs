use maplit::hashset;
use ndarray::Axis;
use std::ops::Mul;
use utilities::{char_matrix, M};
const VERSION: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn peek((y, x): (i64, i64), direction: Direction) -> (i64, i64) {
    match direction {
        Direction::UP => (y - 1, x),
        Direction::RIGHT => (y, x + 1),
        Direction::DOWN => (y + 1, x),
        Direction::LEFT => (y, x - 1),
    }
}
fn turn(prev_direction: Direction) -> Direction {
    match prev_direction {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

#[tokio::main]
async fn main() {
    let day = VERSION
        .strip_prefix("day")
        .unwrap_or_default()
        .parse()
        .unwrap_or(1);
    let content = utilities::get_input(day).await;
    let mut m = char_matrix(content).unwrap();
    let guard_pos: (usize, usize) = m
        .indexed_iter()
        .find(|(_, c)| match c {
            '^' => true,
            _ => false,
        })
        .unwrap()
        .0;
    let guard_pos = (guard_pos.0 as i64, guard_pos.1 as i64);

    let y_len = m.len_of(Axis(0));
    let x_len = m.len_of(Axis(1));
    let mut result = 0;
    for y in 0..y_len {
        println!("{} of {}", y, y_len);
        for x in 0..x_len {
            let restore_char = m[(y, x)];
            m[(y, x)] = '#';
            if is_infinite_loop(guard_pos, &m) {
                result += 1;
            }
            m[(y, x)] = restore_char;
        }
    }

    println!("Solution: {}", result);
}

fn is_infinite_loop(mut guard_pos: (i64, i64), matrix: &M) -> bool {
    let y_len = matrix.len_of(Axis(0));
    let x_len = matrix.len_of(Axis(1));
    let mut d = Direction::UP;
    let mut visited = vec![false; 4 * y_len * x_len];
    while 0 <= guard_pos.0
        && guard_pos.0 < matrix.len_of(Axis(0)) as i64
        && 0 <= guard_pos.1
        && guard_pos.1 < matrix.len_of(Axis(1)) as i64
    {
        if visited[d as usize * y_len * x_len + guard_pos.0 as usize * x_len + guard_pos.1 as usize]
        {
            return true;
        }
        visited[d as usize * y_len * x_len + guard_pos.0 as usize * x_len + guard_pos.1 as usize] =
            true;
        while matrix.get((peek(guard_pos, d).0 as usize, peek(guard_pos, d).1 as usize))
            == Some(&'#')
        {
            d = turn(d);
        }
        guard_pos = peek(guard_pos, d);
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
