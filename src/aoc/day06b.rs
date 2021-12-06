use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> u64 {
    solve_file(&file::input("input06.txt"))
}

fn solve_file(f: &Path) -> u64 {
    let fishes = read_input(f);
    let mut counts = cout_fish_ages(fishes);
    for i in 0..256 {
        counts = generation(counts)
    }
    count_fishes(&counts)
}

fn count_fishes(counts: &Vec<u64>) -> u64 {
    let mut sum = 0;
    for count in counts {
        sum = sum + count
    }
    sum
}

fn cout_fish_ages(fishes: Vec<i32>) -> Vec<u64> {
    let mut ages = vec![0; 9];
    for fish in fishes {
        ages[fish as usize] = ages[fish as usize] + 1
    }
    ages
}

fn generation(ages: Vec<u64>) -> Vec<u64> {
    let mut newages = vec![0; 9];
    for age in 1..9 {
        newages[age - 1] = ages[age];
    }
    newages[8] = ages[0];
    newages[6] = newages[6] + ages[0];
    newages
}

fn read_input(f: &Path) -> Vec<i32> {
    let mut lines: Vec<String> = read_lines(f);
    let draws = read_number_line(&lines[0]);
    draws
}

fn read_lines(p0: &Path) -> Vec<String> {
    let input = file::read_lines(p0).unwrap();
    let mut lines = Vec::new();
    for line in input {
        lines.push(line.unwrap())
    }
    lines
}

fn read_number_line(line: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (\d+)\s*,?\s*
            "
        )
        .unwrap();
    }

    let mut v: Vec<i32> = Vec::new();

    for cap in RE.captures_iter(line) {
        v.push(cap[1].parse().unwrap());
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 87456);
    }

    #[test]
    fn sample() {
        let f = (&file::input("input06-sample.txt"));
        let fishes = read_input(f);
        let mut counts = cout_fish_ages(fishes);
        for i in 0..18 {
            counts = generation(counts)
        }
        let c = count_fishes(&counts);
        assert_eq!(c, 26);
    }
}
