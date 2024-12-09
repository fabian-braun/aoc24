use ndarray::Axis;
use std::u32;
use utilities::char_matrix;

const VERSION: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    let day = VERSION
        .strip_prefix("day")
        .unwrap_or_default()
        .parse()
        .unwrap_or(1);
    let content = utilities::get_example(day).await;
    println!("Example Solution for day {}: \n{:?}\n", day, run2(content));
    let content = utilities::get_input(day).await;
    println!("Actual Solution for day {}: \n{:?}\n", day, run2(content));
}

fn run(input: String) -> anyhow::Result<String> {
    let mut file = vec![];
    let mut id = 0_u16;
    let mut space = false;
    let input = input.split_ascii_whitespace().next().unwrap();
    input.chars().for_each(|c| {
        let count: u8 = format!("{}", c).parse().unwrap_or(0);
        if space {
            for _ in 0..count {
                file.push(u16::MAX);
            }
        } else {
            for _ in 0..count {
                file.push(id);
            }
            id += 1;
        }
        space = !space;
    });
    let mut lc = 0;
    let mut rc = file.len() - 1;
    while lc < rc {
        while file[lc] != u16::MAX && lc < rc {
            lc += 1;
        }
        while file[rc] == u16::MAX && lc < rc {
            rc -= 1;
        }
        file.swap(lc, rc);
        lc += 1;
        rc -= 1;
    }
    let checksum: usize = file
        .iter()
        .filter(|x| **x != u16::MAX)
        .enumerate()
        .map(|(idx, id)| idx * (*id as usize))
        .sum();
    Ok(format!("{:?}", checksum))
}

fn run2(input: String) -> anyhow::Result<String> {
    let mut file = vec![];
    let mut id = 0_u16;
    let mut space = false;
    let input = input.split_ascii_whitespace().next().unwrap();
    input.chars().for_each(|c| {
        let count: u8 = format!("{}", c).parse().unwrap();
        if space {
            file.push((count, u16::MAX));
        } else {
            file.push((count, id));
            id += 1;
        }
        space = !space;
    });
    // println!("{:?}", file);
    let mut id_to_shift = id - 1;
    while id_to_shift > 0 {
        // find next element for a shift attempt
        // find first space block where it fits
        // perform the shift
        // merge space blocks
        let idx_s_e = file
            .iter()
            .position(|(_, idx)| *idx == id_to_shift)
            .unwrap();
        let mut lc = 0;
        while (file[lc].1 != u16::MAX || file[lc].0 < file[idx_s_e].0) && lc < idx_s_e {
            lc += 1;
        }
        if file[lc].1 == u16::MAX && file[lc].0 >= file[idx_s_e].0 && lc < idx_s_e {
            let new_space = (file[idx_s_e].0, u16::MAX);
            file[lc].0 -= file[idx_s_e].0;
            let elem = file.remove(idx_s_e);
            file.insert(idx_s_e, new_space);
            file.insert(lc, elem);
        }
        let mut i = lc;
        loop {
            if i >= idx_s_e || i >= file.len() {
                break;
            }
            if file[i].0 == 0 {
                file.remove(i);
            } else if file[i - 1].1 == file[i].1 {
                file[i - 1].0 += file[i].0;
                file.remove(i);
            } else {
                i += 1;
            }
        }
        id_to_shift = id_to_shift.checked_sub(1).unwrap_or(0);
    }
    // println!("{:?}", file);
    let mut idx = 0_u64;
    let mut checksum = 0_u64;
    for (count, id) in file {
        for _ in 0..count {
            if id != u16::MAX {
                // print!("{:?}", id);
                checksum += idx * (id as u64);
            }
            idx += 1;
        }
    }
    // println!();
    Ok(format!("{:?}", checksum))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
