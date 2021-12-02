use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input02.txt"))
}

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}


fn solve_file(f: &Path) -> i32 {
    let numbers = read_input(f);
    let count = follow(&numbers);
    count.0*count.1
}

fn read_input(f: &Path) -> Vec<Direction> {
    let lines = file::read_lines(f).unwrap();
    let mut directions = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let direction: Direction = parse(&line).unwrap();
        directions.push(direction);
    };
    directions
}


fn parse(line: &str) -> Result<Direction, &str> {
    lazy_static! {
        static ref REF: Regex = Regex::new(r"forward (\d+)").unwrap();
        static ref REU :Regex= Regex::new(r"up (\d+)").unwrap();
        static ref RED :Regex= Regex::new(r"down (\d+)").unwrap();
    }
    match REF.captures(line) {
        Some(m) => {
            Ok(Direction::Forward(m[1].parse().unwrap()))
        }
        None => {
            match REU.captures(line) {
                Some(m) => Ok(Direction::Up(m[1].parse().unwrap())),
                None => {
                    match RED.captures(line) {
                        Some(m) => Ok(Direction::Down(m[1].parse().unwrap())),
                        None => Err("syntax")
                    }
                }
            }
        }
    }
}


fn follow (numbers: &Vec<Direction>) -> (i32,i32) {
    let mut pos = (0,0);
    for num in numbers {
        match num {
            Direction::Forward(f) => {  pos = (pos.0+f, pos.1)}
            Direction::Down(d) => {  pos = (pos.0, pos.1+d)}
            Direction::Up(u) => {  pos = (pos.0, pos.1-u)}
        }
    };
    pos
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 1868935);
    }
}