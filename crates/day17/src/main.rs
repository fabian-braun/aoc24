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
    // let content = utilities::get_example(day).await;
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

fn run(input: String) -> anyhow::Result<String> {
    let mut lines = input.lines();
    let _a: u64 = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let _b: u64 = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let _c: u64 = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let _ = lines.next();
    let p: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect_vec();
    let a = valid_values_for_a(&p, &p);
    Ok(format!("{:?}", a.into_iter().min()))
}

// Program: 2,4, 1,1, 7,5, 0,3, 1,4, 4,5, 5,5, 3,0

fn valid_values_for_a(p: &[u64], expected_output: &[u64]) -> Vec<u64> {
    if expected_output.len() == 1 {
        (0u64..8u64)
            .filter(|&a| test_program_produces_output(&p, &expected_output, (a, 0, 0)))
            .collect_vec()
    } else {
        let a_candidates = valid_values_for_a(&p, &expected_output[1..]);
        println!("found candidates for a: {:?}", a_candidates);
        let mut valid_a = vec![];
        for a_candidate in a_candidates {
            for offset in 0u64..8u64 {
                let a = a_candidate * 8 + offset;
                if test_program_produces_output(&p, &expected_output, (a, 0, 0)) {
                    valid_a.push(a);
                }
            }
        }
        valid_a
    }
}

fn test_program_produces_output(
    p: &[u64],
    expected_output: &[u64],
    mut register: (u64, u64, u64),
) -> bool {
    let mut output = vec![];
    let mut instr_ptr = 0;
    while instr_ptr < p.len() {
        let opcode = p[instr_ptr];
        let arg = p[instr_ptr + 1];
        match opcode {
            0 => {
                register = adv(register, arg);
                instr_ptr += 2;
            }
            1 => {
                register = bxl(register, arg);
                instr_ptr += 2;
            }
            2 => {
                register = bst(register, arg);
                instr_ptr += 2;
            }
            3 => {
                if let Some(ptr) = jnz(register, arg) {
                    instr_ptr = ptr;
                } else {
                    instr_ptr += 2;
                };
            }
            4 => {
                register = bxc(register);
                instr_ptr += 2;
            }
            5 => {
                let (r, reg) = out(register, arg);
                register = reg;
                output.push(r);
                instr_ptr += 2;
            }
            // 6 => {
            //     register = bdv(register, arg);
            //     instr_ptr += 2;
            // }
            7 => {
                register = cdv(register, arg);
                instr_ptr += 2;
            }
            _ => {
                panic!("invalid opcode");
            }
        }
    }
    output == expected_output
}

fn resolve(reg: (u64, u64, u64), c_o: u64) -> u64 {
    match c_o {
        0..=3 => c_o,
        4 => reg.0,
        5 => reg.1,
        6 => reg.2,
        _ => panic!("invalid operand"),
    }
}

fn adv(reg: (u64, u64, u64), c_o: u64) -> (u64, u64, u64) {
    let c_o = resolve(reg, c_o);
    let numerator = reg.0;
    let denominator = 2_u64.checked_pow(c_o as u32).unwrap();
    let result = numerator / denominator;
    (result, reg.1, reg.2)
}

fn bxl(reg: (u64, u64, u64), l_o: u64) -> (u64, u64, u64) {
    let result = l_o as u64 ^ reg.1;
    (reg.0, result, reg.2)
}

fn bst(reg: (u64, u64, u64), c_o: u64) -> (u64, u64, u64) {
    let c_o = resolve(reg, c_o);
    let result = c_o % 8;
    (reg.0, result, reg.2)
}

fn jnz(reg: (u64, u64, u64), l_o: u64) -> Option<usize> {
    if reg.0 == 0 {
        None
    } else {
        Some(l_o as usize)
    }
}

fn bxc(reg: (u64, u64, u64)) -> (u64, u64, u64) {
    let result = reg.1 ^ reg.2;
    (reg.0, result, reg.2)
}

fn out(reg: (u64, u64, u64), c_o: u64) -> (u64, (u64, u64, u64)) {
    let c_o = resolve(reg, c_o);
    let result = c_o % 8;
    (result, reg)
}

// fn bdv(reg: (u64, u64, u64), c_o: u64) -> (u64, u64, u64) {
//     let c_o = resolve(reg, c_o);
//     let numerator = reg.0;
//     let denominator = 2_u64.checked_pow(c_o as u32).unwrap();
//     let result = numerator / denominator;
//     (reg.0, result, reg.2)
// }

fn cdv(reg: (u64, u64, u64), c_o: u64) -> (u64, u64, u64) {
    let c_o = resolve(reg, c_o);
    let numerator = reg.0;
    let denominator = 2_u64.checked_pow(c_o as u32).unwrap();
    let result = numerator / denominator;
    (reg.0, reg.1, result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert!(test_program_produces_output(
            &[0, 3, 5, 4, 3, 0],
            &[0, 3, 5, 4, 3, 0],
            (117440, 0, 0)
        ))
    }
}
