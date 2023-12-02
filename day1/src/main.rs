use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

fn get_number(line: &str) -> u32 {
    let first = line
        .chars()
        .filter_map(|x| x.to_digit(10))
        .next()
        .expect("no number found");
    let second = line
        .chars()
        .rev()
        .filter_map(|x| x.to_digit(10))
        .next()
        .unwrap_or(first);
    first * 10 + second
}

fn get_number2(line: &str) -> u32 {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut idx = line.len() - 1;
    let mut first = line
        .char_indices()
        .filter_map(|(i, c)| {
            idx = i;
            c.to_digit(10)
        })
        .next();

    let mut min_i = idx;
    for (i, digit) in digits.iter().enumerate() {
        if let Some(f) = line[..=idx].find(digit) {
            if f < min_i {
                min_i = f;
                first = Some(i as u32 + 1);
            }
        }
    }

    let mut second = line
        .char_indices()
        .rev()
        .filter_map(|(i, c)| {
            idx = i;
            c.to_digit(10)
        })
        .next();
    if second.is_none() {
        idx = 0;
    }
    let mut max_i = 0;
    for (i, digit) in digits.iter().enumerate() {
        if let Some(f) = line[idx..].rfind(digit) {
            if f > max_i {
                max_i = f;
                second = Some(i as u32 + 1);
            }
        }
    }
    match (first, second) {
        (Some(x), Some(y)) => x * 10 + y,
        (Some(x), None) => x,
        (None, Some(x)) => x,
        (None, None) => 0,
    }
}
fn main() {
    let mut f = File::open("input.txt").unwrap();
    let start = std::time::Instant::now();
    let sum = BufReader::new(&f)
        .lines()
        .fold(0, |acc, x| acc + get_number(&x.unwrap()));
    let elapsed = start.elapsed();
    println!("{} in {:?}", sum, elapsed);

    f.seek(SeekFrom::Start(0)).unwrap();
    let start = std::time::Instant::now();
    let sum = BufReader::new(f)
        .lines()
        .fold(0, |acc, x| acc + get_number2(&x.unwrap()));
    let elapsed = start.elapsed();
    println!("{} in {:?}", sum, elapsed);
}

mod tests {
    use super::*;
    #[test]
    fn test_get_number() {
        let strings = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let sum = strings.iter().fold(0, |acc, x| acc + get_number(x));
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_get_number2() {
        let strings = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "fdsdfonesse",
        ];
        let sum = strings.iter().fold(0, |acc, x| acc + get_number2(x));
        assert_eq!(sum, 281);
    }
}
