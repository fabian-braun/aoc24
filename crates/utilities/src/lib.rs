use anyhow::Context;
use ndarray::Array2;
use reqwest::header;
use reqwest::header::HeaderMap;
use std::fs::{create_dir_all, read_to_string, File};
use std::{env, io};

pub type M = Array2<char>;

pub async fn get_example(day: usize) -> String {
    let file_name = example_file_name(day);
    if File::open(&file_name).is_err() {
        download_example(day).await;
    }
    read_to_string(file_name).unwrap()
}

pub async fn get_input(day: usize) -> String {
    let file_name = input_file_name(day);
    if File::open(&file_name).is_err() {
        download_input(day).await;
    }
    read_to_string(file_name).unwrap()
}

fn input_file_name(day: usize) -> String {
    format!("data/input_day{day}.txt")
}

fn example_file_name(day: usize) -> String {
    format!("data/example_day{day}.txt")
}

async fn download_example(day: usize) {
    _ = create_dir_all("data");
    let resp = reqwest::get(format!("https://adventofcode.com/2024/day/{day}"))
        .await
        .expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let mut example = "".to_string();
    let mut example_started = false;
    for line in body.lines() {
        if let Some(first_line) = line.strip_prefix("<pre><code>") {
            example_started = true;
            example.push_str(first_line);
            example.push_str("\n");
        } else if let Some(last_line) = line.strip_suffix("</code></pre>") {
            example.push_str(last_line);
            break;
        } else if example_started {
            example.push_str(line);
            example.push_str("\n");
        }
    }
    let mut out = File::create(example_file_name(day)).expect("failed to create file");
    io::copy(&mut example.as_bytes(), &mut out).expect("failed to write file");
}

async fn download_input(day: usize) {
    _ = create_dir_all("data");
    let session_cookie = env::var("AOC_SESSION_TOKEN").unwrap().to_string();
    let mut request_headers = HeaderMap::new();
    request_headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&format!("session={}", session_cookie)).unwrap(),
    );
    let client = reqwest::ClientBuilder::new()
        .default_headers(request_headers)
        .build()
        .unwrap();
    let resp = client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .send()
        .await
        .expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let mut out = File::create(input_file_name(day)).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to write file");
}

pub fn char_matrix(raw: String) -> anyhow::Result<M> {
    let y_len = raw.lines().count();
    let x_len = raw.lines().next().context("char_matrix")?.len();
    let mut m = M::default((y_len, x_len));
    raw.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            m[(y, x)] = c;
        })
    });
    Ok(m)
}

pub fn flat_idx_3d((a, b, c): (usize, usize, usize), b_len: usize, c_len: usize) -> usize {
    a * b_len * c_len + b * c_len + c
}

pub fn rev_idx_3d(i: usize, b_len: usize, c_len: usize) -> (usize, usize, usize) {
    let c = i % c_len;
    let b = (i / c_len) % b_len;
    let a = i / (c_len * b_len);
    (a, b, c)
}

#[cfg(test)]
mod tests {
    use crate::{flat_idx_3d, rev_idx_3d};

    #[test]
    fn test_flat_idx() {
        struct TestCase {
            input: (usize, usize, usize),
            dims: (usize, usize, usize),
        }
        let tests = [
            TestCase {
                input: (0, 0, 0),
                dims: (10, 20, 30),
            },
            TestCase {
                input: (2, 4, 3),
                dims: (4, 10, 6),
            },
            TestCase {
                input: (1, 2, 3),
                dims: (10, 20, 30),
            },
            TestCase {
                input: (1, 2, 3),
                dims: (2, 3, 4),
            },
        ];
        for test in tests {
            assert_eq!(
                test.input,
                rev_idx_3d(
                    flat_idx_3d(test.input, test.dims.1, test.dims.2),
                    test.dims.1,
                    test.dims.2
                )
            )
        }
    }
}
