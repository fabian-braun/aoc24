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
    println!("Example Solution for day {}: \n{:?}\n", day, run(content));
    let content = utilities::get_input(day).await;
    println!("Actual Solution for day {}: \n{:?}\n", day, run(content));
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
    let checksum: usize = file.iter().filter(|x| **x != u16::MAX).enumerate().map(|(idx, id)| idx * (*id as usize)).sum();
    Ok(format!("{:?}", checksum))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
