use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use std::fs::{create_dir_all, read_to_string, File};
use std::io;

type M = Array2<char>;
pub async fn get_example(day: usize) -> String {
    let file_name = example_file_name(day);
    if File::open(&file_name).is_err() {
        download_example(day).await;
    }
    read_to_string(file_name).unwrap()
}

pub async fn get_input(day: usize) -> String {
    let file_name = input_file_name(day);
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

pub fn char_matrix(raw: String) -> ArrayBase<OwnedRepr<char>, Ix2> {
    let y_len = raw.lines().count();
    let x_len = raw.lines().next().unwrap().len();
    let mut m = M::default((y_len, x_len));
    raw.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            m[(y, x)] = c;
        })
    });
    m
}
