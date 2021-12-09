use std::path::Path;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input09.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let mut sizes: Vec<u32> = vec![];
    let mut heights = read_file(f);
    loop {
        match get_low_point(&heights) {
            None => {
                break;
            }
            Some((x, y)) => {
                sizes.push(raise_basin(&mut heights, x, y));
            }
        };
    }
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

fn raise_basin(a: &mut Vec<Vec<i8>>, x: usize, y: usize) -> u32 {
    if !is_deep_or_flat(a, x, y) {
        0
    } else {
        a[y][x] = 9;
        1 + if x > 0 { raise_basin(a, x - 1, y) } else { 0 }
            + if y > 0 { raise_basin(a, x, y - 1) } else { 0 }
            + if x < a[y].len() - 1 {
                raise_basin(a, x + 1, y)
            } else {
                0
            }
            + if y < a.len() - 1 {
                raise_basin(a, x, y + 1)
            } else {
                0
            }
    }
}

fn get_low_point(a: &Vec<Vec<i8>>) -> Option<(usize, usize)> {
    let mut ret: Option<(usize, usize)> = None;
    for y in 0..a.len() {
        for x in 0..a[y].len() {
            if is_deep(a, x, y) {
                ret = Some((x, y));
                break;
            }
        }
    }
    ret
}

fn is_deep(a: &Vec<Vec<i8>>, x: usize, y: usize) -> bool {
    (if x > 0 { a[y][x] < a[y][x - 1] } else { true })
        && (if y > 0 { a[y][x] < a[y - 1][x] } else { true })
        && (if x < a[y].len() - 1 {
            a[y][x] < a[y][x + 1]
        } else {
            true
        })
        && (if y < a.len() - 1 {
            a[y][x] < a[y + 1][x]
        } else {
            true
        })
}

fn is_deep_or_flat(a: &Vec<Vec<i8>>, x: usize, y: usize) -> bool {
    a[y][x] < 9
        && (if x > 0 { a[y][x] <= a[y][x - 1] } else { true })
        && (if y > 0 { a[y][x] <= a[y - 1][x] } else { true })
        && (if x < a[y].len() - 1 {
            a[y][x] <= a[y][x + 1]
        } else {
            true
        })
        && (if y < a.len() - 1 {
            a[y][x] <= a[y + 1][x]
        } else {
            true
        })
}

fn read_file(p0: &Path) -> Vec<Vec<i8>> {
    let input = file::read_lines(p0).unwrap();
    input.map(|l| parse_line(&l.expect("fail"))).collect()
}

fn parse_line(line: &str) -> Vec<i8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 1056330); // 282880 too low
    }

    #[test]
    fn sample() {
        let result = solve_file(&file::input("input09-sample.txt"));
        println!("result : {}", result);
        assert_eq!(result, 1134);
    }
}
