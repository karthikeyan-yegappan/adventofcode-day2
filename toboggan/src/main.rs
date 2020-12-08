use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // --snip--
    println!("In file {}", "day2-input.txt");
    let reader = BufReader::new(File::open("day2-input.txt").expect("Cannot open file.txt"));

    // println!("With text:\n{}", contents);

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            println!("word '{}'", word);
        }
        println!("end of each line");
    }

}
