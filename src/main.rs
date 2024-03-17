use std::{
    fs::{self, File},
    io::Read,
    time::Instant,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Metrics {
    // timestamp: String,
    // level_name: String,
    value: u32,
}

fn main() {
    let start = Instant::now();
    let dir = fs::read_dir("D:\\Project\\log-parser\\metrics").unwrap();

    let mut counter: u32 = 0;

    for file in dir {
        let mut content = String::new();
        File::open(file.unwrap().path())
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        let result: Vec<Metrics> = serde_json::from_str(&content).unwrap();

        for item in result {
            counter += item.value;
        }
    }
    println!("{}", counter);
    let duration = start.elapsed();

    println!("time elasped: {:?}", duration);
}
