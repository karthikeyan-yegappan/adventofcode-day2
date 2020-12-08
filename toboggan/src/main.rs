use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;

fn main() {
    // --snip--
    println!("In file {}", "day2-input.txt");
    println!("{}", "PUZZLE - 1");
    _puzzle1_password();
    println!();
    println!("{}", "PUZZLE - 2");
    _puzzle2_password();
    println!();
    _puzzle3_assert();
}

fn _puzzle1_password() {
    let reader = BufReader::new(File::open("day2-input.txt").expect("Cannot open file.txt"));
    let mut _count = 0;
    let mut _total_passwords = 0;
    for line in reader.lines() {
         _count += _string_letter_counts(line.unwrap()); 
         _total_passwords += 1;
     }
     println!("Total passwords: {}",_total_passwords);
     println!("Correct passwords: {}",_total_passwords - _count);
     println!("Incorrect passwords: {}",_count);
}

fn _puzzle2_password() {
    let _reader = BufReader::new(File::open("day2-input.txt").expect("Cannot open file.txt"));
    let mut _count = 0;
    let mut _total_passwords = 0;

    for line in _reader.lines() {
        let words = String::from(line.unwrap());
        let v: Vec<&str> = words.split(' ').collect();
        let _low_high = v[0];
        let mut _password_letter = v[1].as_bytes()[0] as char;
        let mut _password = v[2];
        let numbers: Vec<&str> = _low_high.split('-').collect();
        let mut _first: usize = numbers[0].parse().unwrap_or(0);
        let mut _last: usize = numbers[1].parse().unwrap_or(0);
        let first_char_in_position = _password.as_bytes()[_first - 1] as char;
        let last_char_in_position = _password.as_bytes()[_last - 1] as char;
       
        /*
            1-3 a: abcde is valid: position 1 contains a and position 3 does not.
            1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
            2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
            ***Kept me thinking - should last_char_in_position match _password_letter even if the first does not match ?
        */

        if first_char_in_position == _password_letter && first_char_in_position != last_char_in_position {
            _count += 1;
        } 
        
        if last_char_in_position == _password_letter && first_char_in_position != last_char_in_position {
            _count += 1;
        }
 
         _total_passwords += 1;
     }
    
     println!("Total passwords: {}",_total_passwords);
     println!("Correct passwords: {}",_count);
     println!("Incorrect passwords: {}",_total_passwords - _count);
}

fn _string_letter_counts(line: String) -> i32 {
    let words = String::from(line);
    let v: Vec<&str> = words.split(' ').collect();
    let _low_high = v[0];
    let mut _password_letter = String::from(v[1]);
    let mut _password = v[2];
    let numbers: Vec<&str> = _low_high.split('-').collect();
    let _low: i32 = numbers[0].parse().unwrap_or(0);
    let _high: i32 = numbers[1].parse().unwrap_or(0);
    let _letter_counts: i32 = _password.matches(_password_letter.remove(0)).count().try_into().unwrap();
    if _letter_counts > _high || _letter_counts < _low || _letter_counts == 0 {
        return 1;
    }
    return 0;
}

fn _puzzle3_assert() {
    let lines: Vec<&str> = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc", "1-4 d: abcdefg"].to_vec();
    let mut _count = 0;
    let mut _total_passwords = lines.len();

    for line in lines {
        let words = String::from(line);
        let v: Vec<&str> = words.split(' ').collect();
        let _low_high = v[0];
        let mut _password_letter = v[1].as_bytes()[0] as char;
        let mut _password = v[2];
        let numbers: Vec<&str> = _low_high.split('-').collect();
        let mut _first: usize = numbers[0].parse().unwrap_or(0);
        let mut _last: usize = numbers[1].parse().unwrap_or(0);
        let first_char_in_position = _password.as_bytes()[_first - 1] as char;
        let last_char_in_position = _password.as_bytes()[_last - 1] as char;

        if first_char_in_position == _password_letter && first_char_in_position != last_char_in_position {
            _count += 1;
        } 
        
        if last_char_in_position == _password_letter && first_char_in_position != last_char_in_position {
            _count += 1;
        }

    }

    println!("Total passwords: {}",_total_passwords);
    println!("Correct passwords: {}",_count);
    println!("Incorrect passwords: {}",_total_passwords - _count);
}
