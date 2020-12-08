use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;

fn main() {
    // --snip--
    println!("In file {}", "day2-input.txt");
    let reader = BufReader::new(File::open("day2-input.txt").expect("Cannot open file.txt"));
    let mut _count = 0;
    let mut _total_passwords = 0;

    for line in reader.lines() {
       let words = String::from(line.unwrap());
       let v: Vec<&str> = words.split(' ').collect();
       let _low_high = v[0];
       let mut _password_letter = String::from(v[1]);
       let mut _password = v[2];
       let numbers: Vec<&str> = _low_high.split('-').collect();
       let _low: i32 = numbers[0].parse().unwrap_or(0);
       let _high: i32 = numbers[1].parse().unwrap_or(0);
       let _letter_counts: i32 = _password.matches(_password_letter.remove(0)).count().try_into().unwrap();
      
       if _letter_counts > _high || _letter_counts < _low || _letter_counts == 0 {
            _count = _count + 1;
        }

        _total_passwords += 1;
    }

    println!("Wrong passwords: {}",_count);
    println!("Total passwords: {}",_total_passwords);
    println!("Correct passwords: {}",_total_passwords - _count);
}
