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

fn get_values(numbers: &Vec<i32>) -> i32 {
    let o2 = get_oxygen(numbers, 12);
    let co2 = get_co2(numbers, 12);
    o2 * co2
}

fn read_input(f: &Path) -> Vec<i32> {
    let lines = file::read_lines(f).unwrap();
    let mut numbers = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let number: i32 = i32::from_str_radix(&line, 2).unwrap();
        numbers.push(number);
    };
    numbers
}

fn get_frequencies(numbers: &Vec<i32>) -> Vec<i32> {
    let mut freq_1bits: Vec<i32> = vec![0; 12];
    for num in numbers {
        for bit in 0..12 {
            let mask = 1 << bit;
            if (num & mask) != 0 {
                let count: i32 = match freq_1bits.get(bit) {
                    Some(x) => x + 1,
                    None => 1
                };
                freq_1bits[bit] = count;
            }
        }
    }
    freq_1bits
}

fn get_oxygen(numbers: &Vec<i32>, nbits: usize) -> i32 {
    let mut rest = numbers.clone();
    for bit in (0..nbits).rev() {
        if rest.len() > 1 {
            let freq_1bits: Vec<i32> = get_frequencies(&rest);
            let freq = freq_1bits[bit];
            let most_common = if freq * 2 >= (rest.len()) as i32 { 1 } else { 0 };
            let mask = 1 << bit;
            let compare_mask = if most_common == 1 { mask } else { 0 };
            let mut new_rest = vec![];
            for n in rest {
                if (n & mask) == compare_mask {
                    new_rest.push(n)
                }
            }
            rest = new_rest;
        }
    }
    rest[0]
}

fn get_co2(numbers: &Vec<i32>, nbits: usize) -> i32 {
    let mut rest = numbers.clone();
    for bit in (0..nbits).rev() {
        if rest.len() > 1 {
            let freq_1bits: Vec<i32> = get_frequencies(&rest);
            let freq = freq_1bits[bit];
            let least_common = if freq * 2 >= (rest.len()) as i32 { 0 } else { 1 };
            let mask = 1 << bit;
            let keep_mask = if least_common == 0 { 0 } else { mask };
            let mut new_rest = vec![];
            for n in rest {
                if (n & mask) == keep_mask {
                    new_rest.push(n)
                }
            }
            rest = new_rest;
        }
    }
    rest[0]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 3277956);
    }

    #[test]
    fn o2() {
        let numbers = read_input(&file::input("input03-sample.txt"));
        let o2 = get_oxygen(&numbers, 5);
        println!("result : {}", o2);
        assert_eq!(o2, 23);
    }

    #[test]
    fn co2() {
        let numbers = read_input(&file::input("input03-sample.txt"));
        let co2 = get_co2(&numbers, 5);
        println!("result : {}", co2);
        assert_eq!(co2, 10);
    }
}
