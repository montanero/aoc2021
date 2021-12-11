use std::cmp::{max, min};
use std::path::Path;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input11.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let mut flashes = 0u32;
    let mut field = read_field(f);
    for _ in 0..100 {
        flashes = flashes + iterate_field(&mut field);
    }
    flashes
}

fn iterate_field(field: &mut Vec<Vec<Field>>) -> u32 {
    increment_field(field);
    let counter = flash_field(field);
    reset_flashes(field);
    counter
}

fn increment_field(field: &mut Vec<Vec<Field>>) {
    for line in field {
        for f in line {
            match f {
                Field::Off(v) => *f = Field::Off(*v + 1),
                _ => {}
            }
        }
    }
}

fn reset_flashes(field: &mut Vec<Vec<Field>>) {
    for line in field {
        for f in line {
            match *f {
                Field::Flash => *f = Field::Off(0),
                _ => {}
            }
        }
    }
}

fn flash_field(field: &mut Vec<Vec<Field>>) -> u32 {
    let mut counter = 0u32;
    loop {
        let c = flash_once(field);
        if c == 0 {
            break;
        }
        counter += c;
    }
    counter
}

fn flash_once(field: &mut Vec<Vec<Field>>) -> u32 {
    let mut counter = 0u32;

    for y in 0..field.len() {
        let line_len = field.get(y).unwrap().len();
        for x in 0..line_len {
            let f = field[y][x].clone();
            match f {
                Field::Off(v) => {
                    if v > 9 {
                        flash(field, y, x);
                        counter += 1;
                    }
                }
                _ => {}
            }
        }
    }
    counter
}

fn flash(field: &mut Vec<Vec<Field>>, y: usize, x: usize) {
    for y0 in max(0, y as i32 - 1) as usize..min(10, y + 2) {
        for x0 in max(0, x as i32 - 1) as usize..min(10, x + 2) {
            match field[y0][x0] {
                Field::Off(v) => {
                    field[y0][x0] = Field::Off(v + 1);
                }
                Field::Flash => {}
            }
        }
    }
    field[y][x] = Field::Flash
}

fn read_field(f: &Path) -> Vec<Vec<Field>> {
    let lines = file::read_lines(f).unwrap();
    lines
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| Field::Off(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

#[derive(Clone)]
enum Field {
    Off(u32),
    Flash,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 1603);
    }
}
