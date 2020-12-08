use std::fs;

fn main() {
    // --snip--
    println!("In file {}", "day2-input.txt");

    let contents = fs::read_to_string("day2-input.txt")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
