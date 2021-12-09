use std::path::Path;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input09.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let heights = read_file(f);
    get_levels(heights)
}

fn get_levels(a: Vec<Vec<i8>>) -> i32 {
    let mut sum: i32 = 0;
    for y in 0..a.len() {
        let l = &a[y];
        for x in 0..l.len() {
            let p = &l[x];
            let is_deep = (if x > 0 { p < &l[x - 1] } else { true })
                && (if y > 0 { p < &a[y - 1][x] } else { true })
                && (if x < l.len() - 1 { p < &l[x + 1] } else { true })
                && (if y < a.len() - 1 {
                    p < &a[y + 1][x]
                } else {
                    true
                });
            if is_deep {
                sum = sum + 1 + *p as i32;
            }
        }
    }
    sum
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
        assert_eq!(result, 489);
    }
}
