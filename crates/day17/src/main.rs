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
    let b: u64 = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let c: u64 = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let _ = lines.next();
    let p: Vec<u8> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect_vec();
    for a in 1..100000000 {
        let register = (a, b, c);
        if test_program_is_identity(&p, register) {
            return Ok(a.to_string());
        }
    }
    Ok("shit".to_string())
}

fn test_program_is_identity(p: &[u8], mut register: (u64, u64, u64)) -> bool {
    let mut result_idx = 0;
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
                if r != p[result_idx] as u64 {
                    return false;
                }
                result_idx += 1;
                instr_ptr += 2;
            }
            6 => {
                register = bdv(register, arg);
                instr_ptr += 2;
            }
            7 => {
                register = cdv(register, arg);
                instr_ptr += 2;
            }
            _ => {
                panic!("invalid opcode");
            }
        }
    }
    result_idx == p.len()
}

fn resolve(reg: (u64, u64, u64), c_o: u8) -> u64 {
    match c_o {
        0..=3 => c_o as u64,
        4 => reg.0,
        5 => reg.1,
        6 => reg.2,
        _ => panic!("invalid operand"),
    }
}

fn adv(reg: (u64, u64, u64), c_o: u8) -> (u64, u64, u64) {
    let c_o = resolve(reg, c_o);
    let numerator = reg.0;
    let denominator = 2_u64.checked_pow(c_o as u32).unwrap();
    let result = numerator / denominator;
    (result, reg.1, reg.2)
}

fn bxl(reg: (u64, u64, u64), l_o: u8) -> (u64, u64, u64) {
    let result = l_o as u64 ^ reg.1;
    (reg.0, result, reg.2)
}

fn bst(reg: (u64, u64, u64), c_o: u8) -> (u64, u64, u64) {
    let c_o = resolve(reg, c_o);
    let result = c_o % 8;
    (reg.0, result, reg.2)
}

fn jnz(reg: (u64, u64, u64), l_o: u8) -> Option<usize> {
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

fn out(reg: (u64, u64, u64), c_o: u8) -> (u64, (u64, u64, u64)) {
    let c_o = resolve(reg, c_o);
    let result = c_o % 8;
    (result, reg)
}

fn bdv(reg: (u64, u64, u64), c_o: u8) -> (u64, u64, u64) {
    let c_o = resolve(reg, c_o);
    let numerator = reg.0;
    let denominator = 2_u64.checked_pow(c_o as u32).unwrap();
    let result = numerator / denominator;
    (reg.0, result, reg.2)
}

fn cdv(reg: (u64, u64, u64), c_o: u8) -> (u64, u64, u64) {
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
        assert!(test_program_is_identity(&[0, 3, 5, 4, 3, 0], (117440, 0, 0)))
    }
}
