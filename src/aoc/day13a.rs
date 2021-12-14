use std::collections::HashSet;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> usize {
    solve_file(&file::input("input13.txt"))
}

fn solve_file(f: &Path) -> usize {
    let conns = read_input(f);
    let result = fold(&conns.point, &conns.instr[0]);
    result.len()
}

fn fold(points: &HashSet<Point>, instr: &Instruction) -> HashSet<Point> {
    match instr {
        Instruction::OnX(x) => fold_x(points, *x),
        Instruction::OnY(y) => fold_y(points, *y),
    }
}

fn fold_x(points: &HashSet<Point>, x: i32) -> HashSet<Point> {
    points
        .iter()
        .filter(|p| p.x != x)
        .map(|p| {
            if p.x < x {
                p.clone()
            } else {
                Point {
                    x: x - (p.x - x),
                    y: p.y,
                }
            }
        })
        .collect()
}

fn fold_y(points: &HashSet<Point>, y: i32) -> HashSet<Point> {
    points
        .iter()
        .filter(|p| p.y != y)
        .map(|p| {
            if p.y < y {
                p.clone()
            } else {
                Point {
                    x: p.x,
                    y: y - (p.y - y),
                }
            }
        })
        .collect()
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

enum Instruction {
    OnX(i32),
    OnY(i32),
}

struct Sheet {
    point: HashSet<Point>,
    instr: Vec<Instruction>,
}

fn read_input(f: &Path) -> Sheet {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w+),(\w+)").unwrap();
        static ref REFY: Regex = Regex::new(r"fold along y=(\w+)").unwrap();
        static ref REFX: Regex = Regex::new(r"fold along x=(\w+)").unwrap();
    }
    let lines = file::read_lines(f).unwrap();
    let mut points = HashSet::new();
    let mut instr = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if !line.is_empty() {
            match RE.captures(&line) {
                Some(m) => {
                    let s1 = String::from(&m[1]).parse().unwrap();
                    let s2 = String::from(&m[2]).parse().unwrap();
                    let c = Point { x: s1, y: s2 };
                    points.insert(c);
                }
                None => match REFX.captures(&line) {
                    Some(m) => {
                        let s1 = String::from(&m[1]).parse().unwrap();
                        let c = Instruction::OnX(s1);
                        instr.push(c);
                    }
                    None => match REFY.captures(&line) {
                        Some(m) => {
                            let s1 = String::from(&m[1]).parse().unwrap();
                            let c = Instruction::OnY(s1);
                            instr.push(c);
                        }
                        None => {
                            panic!("fail");
                        }
                    },
                },
            }
        }
    }

    Sheet {
        point: points,
        instr: instr,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 4413);
    }
}
