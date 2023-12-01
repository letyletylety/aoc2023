use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    println!("{}", calc_file(String::new(), true))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// .
fn calc_line(line: &str, is_hard: bool) -> u64 {
    let mut a = 0;

    for (index, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            a = 10 * digit;
            break;
        }
        if is_hard {
            if let Some(digit) = to_digit_eng(&line[index..]) {
                a = 10 * digit;
                break;
            }
        }
    }

    let len = line.len();

    for index in (0..len).rev() {
        let c = line.chars().nth(index).unwrap();
        if let Some(digit) = c.to_digit(10) {
            a += digit;
            break;
        }

        if is_hard {
            if let Some(digit) = to_digit_eng_rev(&line[..index + 1]) {
                a += digit;
                break;
            }
        }
    }

    println!("{}", a);
    a.into()
}

fn calc_file(_: String, is_hard: bool) -> u64 {
    let mut sum: u64 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input/day1_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                sum += calc_line(ip.as_str(), is_hard)
            }
        }
    }
    sum
}

fn to_digit_eng(s: &str) -> Option<u32> {
    let strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (index, string) in strings.iter().enumerate() {
        if s.starts_with(string) {
            return Some(1 + index as u32);
        }
    }
    None
}

fn to_digit_eng_rev(s: &str) -> Option<u32> {
    let strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (index, string) in strings.iter().enumerate() {
        if s.ends_with(string) {
            return Some(1 + index as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::calc_line;

    #[test]
    fn calc_line_test() {
        let result = calc_line("3nineonermn", true);
        assert_eq!(31, result)
    }
}
