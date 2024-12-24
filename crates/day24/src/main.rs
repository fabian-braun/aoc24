use crate::Op::{AND, PANIC, XOR};
use itertools::Itertools;
use maplit::hashmap;
use std::collections::HashMap;
use std::time::Instant;
use Op::OR;

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
    let (starting_values, transitions) = input.split_once("\n\n").unwrap();
    let mut id_to_idx: HashMap<String, usize> = hashmap! {};
    let mut current_values = vec![false; 0];
    let mut is_set = vec![false; 0];
    let mut from = vec![(0_usize, PANIC, 0_usize); 0];
    starting_values
        .split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(idx, id_val)| {
            let (id, val) = id_val.split_once(": ").unwrap();
            id_to_idx.insert(id.to_string(), idx);
            let val: bool = val == "1".to_string();
            current_values.push(val);
            is_set.push(true);
            from.push((0, PANIC, 0));
        });

    transitions
        .split('\n')
        .filter(|l| !l.is_empty())
        .for_each(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let (a, op, b) = left.split(' ').collect_tuple().unwrap();
            let op = Op::from_str(op);
            for &x in &[right, a, b] {
                if !id_to_idx.contains_key(x) {
                    id_to_idx.insert(x.to_string(), current_values.len());
                    current_values.push(false);
                    is_set.push(false);
                    from.push((0, PANIC, 0));
                }
            }
            from[id_to_idx[right]] = (id_to_idx[a], op, id_to_idx[b]);
        });

    let mut result = 0_usize;
    let mut multiplier = 1_usize;
    let mut id_num = 0;
    while let Some(idx) = id_to_idx.get(&format!["z{:02}", id_num].to_string()) {
        compute_value_of(*idx, &mut is_set, &from, &mut current_values);
        result += multiplier * usize::from(current_values[*idx]);
        id_num += 1;
        multiplier *= 2;
    }


    Ok(result.to_string())
}

fn compute_value_of(
    idx: usize,
    is_set: &mut [bool],
    from: &[(usize, Op, usize)],
    current_values: &mut [bool],
) {
    if is_set[idx] {
        return;
    } else {
        let (a, op, b) = from[idx];
        compute_value_of(a, is_set, from, current_values);
        compute_value_of(b, is_set, from, current_values);
        let a_val = current_values[a];
        let b_val = current_values[b];
        current_values[idx] = op.eval(a_val, b_val);
        is_set[idx] = true;
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Op {
    XOR,
    OR,
    AND,
    PANIC,
}

impl Op {
    fn eval(self, a: bool, b: bool) -> bool {
        match self {
            XOR => a ^ b,
            OR => a | b,
            AND => a & b,
            PANIC => {
                panic!("{} {:?} {}", a, self, b)
            }
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "XOR" => Self::XOR,
            "OR" => OR,
            "AND" => Self::AND,
            _ => panic!("{}", s),
        }
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
                content: "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
                expected: "4",
            },
            Example {
                content: "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
                expected: "2024",
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
