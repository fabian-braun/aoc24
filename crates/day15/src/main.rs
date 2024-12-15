use itertools::Itertools;
use ndarray::Axis;
use std::time::Instant;
use utilities::{char_matrix, M};

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
    let (map, movement) = input.split_once("\n\n").unwrap();
    let movement = movement
        .chars()
        .filter_map(|c| match c {
            '<' => Some((0, -1)),
            '>' => Some((0, 1)),
            '^' => Some((-1, 0)),
            'v' => Some((1, 0)),
            _ => None,
        })
        .collect_vec();
    let m = char_matrix(map.to_string())?;
    let mut m = expand(m);
    let mut pos = m
        .indexed_iter()
        .find_or_first(|(_, &c)| c == '@')
        .unwrap()
        .0;
    for mv in movement {
        if mv.0 == 0 {
            // horizontal move
            if push_h(pos, mv, &mut m) {
                pos = (
                    pos.0.checked_add_signed(mv.0).unwrap(),
                    pos.1.checked_add_signed(mv.1).unwrap(),
                )
            }
        } else {
            let d = mv.0;
            // vertical move
            if can_push_v(pos, d, &m) {
                push_v(&[pos], d, &mut m);
                pos = (pos.0.checked_add_signed(d).unwrap(), pos.1)
            }
        }
    }
    let result: usize = m
        .indexed_iter()
        .map(|((y, x), &c)| if c == '[' { 100_usize * y + x } else { 0 })
        .sum();
    Ok(result.to_string())
}

fn can_push_v(pos: (usize, usize), d: isize, m: &M) -> bool {
    assert_ne!(d, 0);
    let tgt = match m[pos] {
        '[' => vec![
            (pos.0.checked_add_signed(d).unwrap(), pos.1),
            (pos.0.checked_add_signed(d).unwrap(), pos.1 + 1),
        ],
        ']' => vec![
            (pos.0.checked_add_signed(d).unwrap(), pos.1),
            (pos.0.checked_add_signed(d).unwrap(), pos.1 - 1),
        ],
        '@' => vec![(pos.0.checked_add_signed(d).unwrap(), pos.1)],
        '.' => return true,
        _ => return false,
    };
    tgt.iter().all(|tgt| can_push_v(*tgt, d, m))
}

fn push_v(pos: &[(usize, usize)], d: isize, m: &mut M) {
    assert_ne!(d, 0);
    let push_next = pos
        .iter()
        .map(|&pos| {
            let tgt = (pos.0.checked_add_signed(d).unwrap(), pos.1);
            match m[tgt] {
                '[' => vec![(tgt.0, tgt.1), (tgt.0, tgt.1 + 1)],
                ']' => vec![(tgt.0, tgt.1 - 1), (tgt.0, tgt.1)],
                _ => vec![],
            }
        })
        .flatten()
        .unique()
        .collect_vec();
    if !push_next.is_empty() {
        push_v(&push_next, d, m);
    }
    pos.iter().for_each(|pos| {
        m[(pos.0.checked_add_signed(d).unwrap(), pos.1)] = m[*pos];
        m[*pos] = '.';
    });
}

fn push_h(pos: (usize, usize), d: (isize, isize), m: &mut M) -> bool {
    let tgt = (
        pos.0.checked_add_signed(d.0).unwrap(),
        pos.1.checked_add_signed(d.1).unwrap(),
    );
    match m[tgt] {
        '.' => {
            m[tgt] = m[pos];
            m[pos] = '.';
            true
        }
        '[' => {
            if push_h(tgt, d, m) {
                m[tgt] = m[pos];
                m[pos] = '.';
                true
            } else {
                false
            }
        }
        ']' => {
            if push_h(tgt, d, m) {
                m[tgt] = m[pos];
                m[pos] = '.';
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn expand(m: M) -> M {
    let mut e = M::default((m.len_of(Axis(0)), m.len_of(Axis(0)) * 2));
    m.indexed_iter().for_each(|((y, x), &c)| {
        let e_x = x * 2;
        match c {
            '#' => {
                e[(y, e_x)] = '#';
                e[(y, e_x + 1)] = '#';
            }
            'O' => {
                e[(y, e_x)] = '[';
                e[(y, e_x + 1)] = ']';
            }
            '.' => {
                e[(y, e_x)] = '.';
                e[(y, e_x + 1)] = '.';
            }
            '@' => {
                e[(y, e_x)] = '@';
                e[(y, e_x + 1)] = '.';
            }
            _ => {}
        }
    });
    e
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        struct Example {
            content: &'static str,
            expected: &'static str,
        }
        let examples = [Example {
            content: "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
            expected: "9021",
        }];
        for (i, ex) in examples.iter().enumerate() {
            assert_eq!(
                ex.expected.to_string(),
                run(ex.content.to_string()).unwrap(),
                "example {} failed:",
                i + 1
            );
        }
    }
}
