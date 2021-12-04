use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input04.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let game = read_input(f);
    let mut winning_number = None;
    let mut winner = None;
    let draws = game.draws;
    let mut boards = game.boards;
    'outer: for draw in draws {
        let mut nboards = Vec::new();
        for mut boardp in boards {
            boardp.check(draw);
            if !boardp.is_winner() {
                nboards.push(boardp)
            } else {
                winning_number = Some(draw);
                winner = Some(boardp)
            }
        }
        boards = nboards;
        if boards.is_empty() {
            break 'outer;
        }
    }
    winner.unwrap().value() * winning_number.unwrap()
}

struct BingoBoard {
    rows: Vec<Vec<i32>>,
    crosses: Vec<Vec<bool>>,
}

impl BingoBoard {
    pub(crate) fn value(&self) -> i32 {
        let mut value = 0;
        for ci in 0..5 {
            for ri in 0..5 {
                if !self.crosses[ri][ci] {
                    value = value + self.rows[ri][ci];
                }
            }
        }
        value
    }
}

impl BingoBoard {
    pub(crate) fn is_winner(&self) -> bool {
        let mut result = false;
        for row in &self.crosses {
            let mut row_result = true;
            for val in row {
                row_result = row_result & val
            }
            result = result | row_result
        }
        if !result {
            for ci in 0..5 {
                let mut col_result = true;
                for ri in 0..5 {
                    col_result = col_result & self.crosses[ri][ci]
                }
                result = result | col_result
            }
        }
        result
    }
}

impl BingoBoard {
    pub(crate) fn check(&mut self, draw: i32) {
        for ir in 0..5 {
            for ic in 0..5 {
                if self.rows[ir][ic] == draw {
                    self.crosses[ir][ic] = true
                }
            }
        }
    }
}

struct BingoFile {
    boards: Vec<BingoBoard>,
    draws: Vec<i32>,
}

fn read_input(f: &Path) -> BingoFile {
    let mut lines: Vec<String> = read_lines(f);
    lines.retain(|line| line.len() != 0);
    let draws = read_draws(&lines[0]);
    lines.remove(0);
    let mut boards: Vec<BingoBoard> = Vec::new();
    while lines.len() != 0 {
        let board: BingoBoard = read_board(&mut lines);
        boards.push(board);
    }
    BingoFile {
        draws: draws,
        boards: boards,
    }
}

fn read_lines(p0: &Path) -> Vec<String> {
    let input = file::read_lines(p0).unwrap();
    let mut lines = Vec::new();
    for line in input {
        lines.push(line.unwrap())
    }
    lines
}

fn read_board(input: &mut Vec<String>) -> BingoBoard {
    let mut lines: Vec<Vec<i32>> = Vec::new();
    let mut crosses: Vec<Vec<bool>> = Vec::new();
    for _ in 0..5 {
        let line = input.remove(0);
        let line = read_numbers(&line);
        lines.push(line);
        crosses.push(vec![false, false, false, false, false])
    }
    BingoBoard {
        rows: lines,
        crosses: crosses,
    }
}

fn read_numbers(line: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (\d+)\s*
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

fn read_draws(line: &str) -> Vec<i32> {
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
}
