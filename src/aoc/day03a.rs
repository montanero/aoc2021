use std::path::Path;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input03.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let numbers = read_input(f);
    let count = get_values(&numbers);
    count
}

fn read_input(f: &Path) -> Vec<i32> {
    let lines = file::read_lines(f).unwrap();
    let mut numbers = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let number: i32 = i32::from_str_radix(&line, 2).unwrap();
        numbers.push(number);
    }
    numbers
}

fn get_values(numbers: &Vec<i32>) -> i32 {
    let mut freq_1bits: Vec<i32> = vec![0; 12];
    for num in numbers {
        for bit in 0..12 {
            let mask = 1 << bit;
            if (num & mask) != 0 {
                let count: i32 = match freq_1bits.get(bit) {
                    Some(x) => x + 1,
                    None => 1,
                };
                freq_1bits[bit] = count;
            }
        }
    }
    let nlines = numbers.len();
    let nbits = freq_1bits.len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..nbits {
        let mask = 1 << bit;
        let f = freq_1bits[bit];
        if f * 2 > nlines as i32 {
            gamma = gamma | mask;
        } else if f * 2 < nlines as i32 {
            epsilon = epsilon | mask;
        }
    }
    epsilon * gamma
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 2498354);
    }
}
