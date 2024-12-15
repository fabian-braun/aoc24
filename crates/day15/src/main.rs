use itertools::Itertools;
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
    let mut m = char_matrix(map.to_string())?;
    let mut pos = m
        .indexed_iter()
        .find_or_first(|(_, &c)| c == '@')
        .unwrap()
        .0;
    for mv in movement {
        if push(pos, mv, &mut m) {
            pos = (
                pos.0.checked_add_signed(mv.0).unwrap(),
                pos.1.checked_add_signed(mv.1).unwrap(),
            )
        }
    }
    let result: usize = m.indexed_iter().map(|((y, x), &c)| {
        if c == 'O' {
            100_usize * y + x
        } else {
            0
        }
    }).sum();
    Ok(result.to_string())
}

fn push(pos: (usize, usize), d: (isize, isize), m: &mut M) -> bool {
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
        'O' => {
            if push(tgt, d, m) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        struct Example {
            content: &'static str,
            expected: &'static str,
        }
        let examples = [
            Example {
                content: "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
                expected: "2028",
            },
            Example {
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
                expected: "10092",
            },
        ];
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
