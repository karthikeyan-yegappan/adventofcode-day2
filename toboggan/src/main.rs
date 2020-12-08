use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // --snip--
    println!("In file {}", "day2-input.txt");
    let reader = BufReader::new(File::open("day2-input.txt").expect("Cannot open file.txt"));
    for line in reader.lines() {
       let words = String::from(line.unwrap());
       let v: Vec<&str> = words.split(' ').collect();
       println!("{:?}", v);
        
    }
   // Ok(());
}
