use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input07.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let positions = read_input(f);
    let min = positions.iter().min().unwrap().clone();
    let max = positions.iter().max().unwrap().clone();
    let mut min_fuel = i32::MAX;
    let mut min_pos = -1;
    for pos in min..max + 1 {
        let fuel = calculate_fuel(&positions, pos);
        if fuel < min_fuel {
            min_fuel = fuel;
            min_pos = pos;
        }
    }
    min_fuel
}

fn calculate_fuel(positions: &Vec<i32>, pos: i32) -> i32 {
    let fuel: i32 = positions.iter().map(|p| (p - pos).abs()).sum();
    fuel
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
        assert_eq!(result, 341534);
    }

    /*    #[test]
        fn sample() {
            let f = (&file::input("input06-sample.txt"));
        }
    */
}
