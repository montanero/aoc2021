use std::cmp::{max, min};
use std::path::Path;

use crate::aoc::file;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input05.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let numbers = read_input(f);
    let f = drawLines(&numbers);
    count_doubles(&f)
}

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

fn read_input(f: &Path) -> Vec<Line> {
    let lines = file::read_lines(f).unwrap();
    let mut numbers = Vec::new();
    for l in lines {
        let l = l.unwrap();
        let line = read_line(&l);
        numbers.push(line);
    }
    numbers
}

fn read_line(str: &str) -> Line {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (\d+)\s*,\s*(\d+)\s*->\s*(\d+)\s*,\s*(\d+)
            "
        )
        .unwrap();
    }
    let cap = match RE.captures(str) {
        Some(x) => x,
        None => panic!("pattern"),
    };
    let start = Point {
        x: cap[1].parse().unwrap(),
        y: cap[2].parse().unwrap(),
    };
    let end = Point {
        x: cap[3].parse().unwrap(),
        y: cap[4].parse().unwrap(),
    };
    Line {
        start: start,
        end: end,
    }
}

fn drawLines(lines: &Vec<Line>) -> Vec<Vec<i32>> {
    let SIZE = 1000;
    let mut field = empty_field(SIZE);
    for line in lines {
        draw_line(&mut field, line);
    }
    field
}

fn draw_line(field: &mut Vec<Vec<i32>>, line: &Line) {
    if (line.start.x == line.end.x) {
        let ymin = min(line.start.y, line.end.y) as usize;
        let ymax = max(line.start.y, line.end.y) as usize;
        for y in ymin..ymax + 1 {
            field[y][line.start.x as usize] = field[y][line.start.x as usize] + 1;
        }
    } else if (line.start.y == line.end.y) {
        let xmin = min(line.start.x, line.end.x) as usize;
        let xmax = max(line.start.x, line.end.x) as usize;
        for x in xmin..xmax + 1 {
            field[line.start.y as usize][x] = field[line.start.y as usize][x] + 1;
        }
    }
}

fn empty_field(size: i32) -> Vec<Vec<i32>> {
    let mut field = Vec::new();
    for i in 0..size {
        field.push(vec![0; size as usize])
    }
    field
}

fn count_doubles(field: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for line in field {
        for place in line {
            if (*place > 1) {
                count = count + 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 5197);
    }

    #[test]
    fn sample() {
        let f = &file::input("input05-sample.txt");
        let numbers = read_input(f);
        let f = drawLines(&numbers);
        let doubles = count_doubles(&f);
        assert_eq!(doubles, 5);
    }
}
