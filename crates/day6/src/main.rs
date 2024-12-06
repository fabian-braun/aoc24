use maplit::hashset;
use ndarray::Axis;
use utilities::char_matrix;

#[derive(Debug, Clone, Copy)]
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
    let content = utilities::get_example(6).await;
    let matrix = char_matrix(content);
    let guard_pos: (usize, usize) = matrix
        .indexed_iter()
        .find(|(_, c)| match c {
            '^' => true,
            _ => false,
        })
        .unwrap()
        .0;
    let mut guard_pos = (guard_pos.0 as i64, guard_pos.1 as i64);
    let mut d = Direction::UP;
    println!("Guard pos: {:?}", guard_pos);
    let mut visited = hashset! {};
    while 0 <= guard_pos.0
        && guard_pos.0 < matrix.len_of(Axis(0)) as i64
        && 0 <= guard_pos.1
        && guard_pos.1 < matrix.len_of(Axis(0)) as i64
    {
        visited.insert(guard_pos);
        while matrix.get((peek(guard_pos, d).0 as usize, peek(guard_pos, d).1 as usize))
            == Some(&'#')
        {
            d = turn(d);
        }
        guard_pos = peek(guard_pos, d);
    }
    println!("Solution: {}", visited.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
