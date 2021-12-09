use std::collections::{HashMap, HashSet};
use std::path::Path;

use itertools::Itertools;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input08.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let pats = read_file(f);
    pats.iter().map(|p| decode(p)).sum()
}

fn decode(p: &Patterns) -> i32 {
    let map = invert(identify(&p.input));
    let digits: Vec<&i32> = p
        .output
        .iter()
        .map(|s| match map.get(s) {
            Some(x) => x,
            None => panic!("fail"),
        })
        .collect();
    digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]
}

fn invert(inp: HashMap<i32, String>) -> HashMap<String, i32> {
    let mut ret = HashMap::new();
    for (k, v) in inp {
        ret.insert(v, k);
    }
    ret
}

fn identify(pats: &Vec<String>) -> HashMap<i32, String> {
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(
        1,
        match pats.iter().find(|p| p.len() == 2) {
            Some(p) => String::from(p),
            None => panic!("fail"),
        },
    );
    map.insert(
        4,
        match pats.iter().find(|p| p.len() == 4) {
            Some(p) => String::from(p),
            None => panic!("fail"),
        },
    );
    map.insert(
        7,
        match pats.iter().find(|p| p.len() == 3) {
            Some(p) => String::from(p),
            None => panic!("fail"),
        },
    );
    map.insert(
        8,
        match pats.iter().find(|p| p.len() == 7) {
            Some(p) => String::from(p),
            None => panic!("fail"),
        },
    );
    map.insert(
        3,
        match pats
            .iter()
            .find(|p| p.len() == 5 && contains_all(p, &map[&1]))
        {
            Some(p) => String::from(p),
            None => panic!("no 3"),
        },
    );
    map.insert(
        9,
        match pats
            .iter()
            .find(|p| p.len() == 6 && contains_all(p, &map[&4]))
        {
            Some(p) => String::from(p),
            None => panic!("no 3"),
        },
    );
    map.insert(
        0,
        match pats
            .iter()
            .find(|p| p.len() == 6 && contains_all(p, &map[&1]) && map[&9] != **p)
        {
            Some(p) => String::from(p),
            None => panic!("no 3"),
        },
    );
    map.insert(
        6,
        match pats
            .iter()
            .find(|p| p.len() == 6 && map[&9] != **p && map[&0] != **p)
        {
            Some(p) => String::from(p),
            None => panic!("no 3"),
        },
    );
    map.insert(
        5,
        match pats
            .iter()
            .find(|p| p.len() == 5 && contains_all(&map[&6], p))
        {
            Some(p) => String::from(p),
            None => panic!("no 3"),
        },
    );
    map.insert(
        2,
        match pats
            .iter()
            .find(|p| p.len() == 5 && map[&3] != **p && map[&5] != **p)
        {
            Some(p) => String::from(p),
            None => panic!("no 3"),
        },
    );
    map
}

fn contains_all(s1: &str, s2: &str) -> bool {
    let ss1: HashSet<char> = HashSet::from_iter(s1.chars());
    let ss2: HashSet<char> = HashSet::from_iter(s2.chars());
    ss1.intersection(&ss2).eq(&ss2)
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
    fn bla(s: &str) -> Vec<String> {
        s.split(" ")
            .map(|r| r.chars().sorted().collect::<String>())
            .collect()
    }
    let q: Vec<&str> = line.split(" | ").collect();
    let inp = bla(q[0]);
    let outp = bla(q[1]);
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
    fn test_identify() {
        let ps = parse_line(
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        );
        let m = identify(&ps.input);
        assert_eq!(m[&0], "abcdfg");
        assert_eq!(m[&1], "cg");
        assert_eq!(m[&2], "abcde");
        assert_eq!(m[&3], "bcdeg");
        assert_eq!(m[&4], "cefg");
        assert_eq!(m[&5], "bdefg");
        assert_eq!(m[&6], "abdefg");
        assert_eq!(m[&7], "bcg");
        assert_eq!(m[&8], "abcdefg");
        assert_eq!(m[&9], "bcdefg");
    }

    #[test]
    fn test_decode() {
        let result = decode(&parse_line(
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        ));
        assert_eq!(result, 9781);
    }

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 1016804);
    }
}
