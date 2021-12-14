use std::collections::HashMap;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input14.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let mut p = read_input(f);
    for _ in 0..10 {
        p = p.apply();
    }
    let h = p.histogram();
    let mut count_vec: Vec<(&char, &u32)> = h.iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(b.1));
    count_vec[count_vec.len() - 1].1 - count_vec[0].1
}

struct Polymere {
    template: String,
    rules: HashMap<String, String>,
}

impl Polymere {
    fn apply(mut self) -> Polymere {
        let mut s = String::from(&self.template[0..1]);
        for i in 0..self.template.len() - 1 {
            let x = &self.template[i..i + 2];
            let ins = self.rules.get(x);
            match ins {
                None => {
                    s += &x[1..2];
                }
                Some(y) => {
                    s += y;
                    s += &x[1..2];
                }
            }
        }
        self.template = s;
        self
    }

    fn histogram(self) -> HashMap<char, u32> {
        let mut m = HashMap::new();
        for c in self.template.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        m
    }
}

fn read_input(f: &Path) -> Polymere {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w+)\s*->\s*(\w+)").unwrap();
        static ref RET: Regex = Regex::new(r"(\w+)").unwrap();
    }
    let lines = file::read_lines(f).unwrap();
    let mut rules = HashMap::new();
    let mut temple = String::new();
    for line in lines {
        let line = line.unwrap();
        if !line.is_empty() {
            match RE.captures(&line) {
                Some(m) => {
                    let s1 = String::from(&m[1]).parse().unwrap();
                    let s2 = String::from(&m[2]).parse().unwrap();
                    rules.insert(s1, s2);
                }
                None => match RET.captures(&line) {
                    Some(m) => {
                        let s1 = String::from(&m[1]);
                        temple = s1;
                    }
                    None => panic!("fail"),
                },
            }
        }
    }
    Polymere {
        template: temple,
        rules: rules,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 3306);
    }
}
