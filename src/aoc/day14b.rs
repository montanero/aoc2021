use std::collections::HashMap;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> u64 {
    solve_file(&file::input("input14.txt"))
}

fn solve_file(f: &Path) -> u64 {
    let mut p = read_input(f);
    p = p.pairs();
    for _ in 0..40 {
        p = p.apply();
    }
    let h = p.histogram();
    let mut count_vec: Vec<(&char, &u64)> = h.iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(b.1));
    count_vec[count_vec.len() - 1].1 - count_vec[0].1
}

struct Polymere {
    template: String,
    rules: HashMap<String, String>,
    pairs: HashMap<String, u64>,
}

impl Polymere {
    fn pairs(mut self) -> Polymere {
        self.pairs.clear();
        for i in 0..self.template.len() - 1 {
            let x = &self.template[i..i + 2];
            *self.pairs.entry(String::from(x)).or_insert(0) += 1;
        }
        self
    }

    fn apply(mut self) -> Polymere {
        let opairs = self.pairs;
        self.pairs = HashMap::new();
        for (k, v) in opairs {
            match self.rules.get(&k) {
                None => {}
                Some(f) => {
                    let t1 = String::from(&k[0..1]) + f;
                    let t2 = f.clone() + &k[1..2];
                    *self.pairs.entry(t1).or_insert(0) += v;
                    *self.pairs.entry(t2).or_insert(0) += v;
                }
            }
        }
        self
    }

    fn histogram(self) -> HashMap<char, u64> {
        let mut m = HashMap::new();
        for p in self.pairs {
            let c = p.0.chars().nth(0).unwrap();
            *m.entry(c).or_insert(0) += p.1
        }
        let last = self.template.chars().nth(self.template.len() - 1).unwrap();
        *m.entry(last).or_insert(0) += 1;
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
        pairs: HashMap::new(),
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
