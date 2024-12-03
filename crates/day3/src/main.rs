#[tokio::main]
async fn main() {
    let content = utilities::get_input(3).await;
    //                  0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   10,  11,
    let pattern = vec!['m', 'u', 'l', '(', '0', '0', '0', ',', '0', '0', '0', ')'];
    let mut idx = 0;
    let mut total = 0i64;
    let mut first_num = "".to_string();
    let mut second_num = "".to_string();
    content.chars().for_each(|c| {
        if idx == 4 {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    first_num += &format!("{}", c);
                }
                ',' => {
                    idx += 1;
                }
                _ => {
                    first_num = "".to_string();
                    second_num = "".to_string();
                    idx = 0;
                }
            }
        } else if idx == 5 {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    second_num += &format!("{}", c);
                }
                ')' => {
                    total += first_num.parse::<i64>().unwrap() * second_num.parse::<i64>().unwrap();
                    first_num = "".to_string();
                    second_num = "".to_string();
                    idx = 0;
                }
                _ => {
                    first_num = "".to_string();
                    second_num = "".to_string();
                    idx = 0;
                }
            }
        } else if c == pattern[idx] {
            idx += 1;
        } else {
            first_num = "".to_string();
            second_num = "".to_string();
            idx = 0;
            if c == pattern[idx] {
                idx += 1;
            }
        }
    });

    println!("Part I solution: {}", total);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
