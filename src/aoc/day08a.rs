use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input08.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let pats = read_file(f);
    count_simple(&pats)
}

fn count_simple(pat: &Vec<Patterns>) -> i32 {
    pat.iter()
        .map(|p| &p.output)
        .map(|o| count_simple_str(o))
        .sum()
}

fn count_simple_str(p0: &Vec<String>) -> i32 {
    p0.iter()
        .filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7)
        .count() as i32
}

struct Patterns {
    input: Vec<String>,
    output: Vec<String>,
}

fn read_file(p0: &Path) -> Vec<Patterns> {
    let input = file::read_lines(p0).unwrap();
    input.map(|l| parse_line(&l.expect("fail"))).collect()
}

fn parse_line(line: &str) -> Patterns {
    let q: Vec<&str> = line.split(" | ").collect();
    let inp = q[0].split(" ").map(|r| String::from(r)).collect();
    let outp = q[1].split(" ").map(|r| String::from(r)).collect();
    Patterns {
        input: inp,
        output: outp,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = parse_line(
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        );
        assert_eq!(result.input.len(), 10);
        assert_eq!(result.output.len(), 4);
    }

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 521);
    }
}
