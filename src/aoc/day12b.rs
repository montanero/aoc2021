use std::collections::HashSet;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input12.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let conns = read_input(f);
    let empty = &HashSet::new();
    ways_from(&conns, "start", "", empty, false).len() as u32
}

fn ways_from(
    network: &Network,
    here: &str,
    path_before: &str,
    visited: &HashSet<&str>,
    twice: bool,
) -> HashSet<String> {
    let path = String::new() + path_before + "->" + here;
    if here == "end" {
        vec![path].into_iter().collect()
    } else {
        let nvisited = if is_small(here) {
            let mut x = visited.clone();
            x.insert(here);
            x
        } else {
            visited.clone()
        };
        let x = network.destinations_from(here);
        let mut rc: HashSet<String> = HashSet::new();
        for now in x.difference(&nvisited) {
            rc.extend(ways_from(network, now, &path, &nvisited, twice).into_iter());
            if !twice && here != "start" && is_small(here) {
                rc.extend(ways_from(network, now, &path, visited, true).into_iter());
            }
        }
        rc
    }
}

fn is_small(p0: &str) -> bool {
    p0.chars()
        .map(|c| c.is_lowercase())
        .reduce(|a, b| a && b)
        .unwrap()
}

struct Connection {
    from: String,
    to: String,
}

struct Network {
    connections: Vec<Connection>,
}

impl Network {
    fn destinations_from(&self, start: &str) -> HashSet<&str> {
        let s1 = self
            .connections
            .iter()
            .filter(|c| c.from == start)
            .map(|c| c.to.as_ref());
        let s2 = self
            .connections
            .iter()
            .filter(|c| c.to == start)
            .map(|c| c.from.as_ref());
        let s3 = s1.chain(s2);
        s3.collect()
    }
}

fn read_input(f: &Path) -> Network {
    let lines = file::read_lines(f).unwrap();
    let mut directions = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let direction: Connection = parse(&line).unwrap();
        directions.push(direction);
    }
    Network {
        connections: directions,
    }
}

fn parse(line: &str) -> Result<Connection, &str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w+)-(\w+)").unwrap();
    }
    match RE.captures(line) {
        Some(m) => {
            let s1 = String::from(&m[1]);
            let s2 = String::from(&m[2]);
            let c = Connection { from: s1, to: s2 };
            Ok(c)
        }
        None => Err("fail"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 118803);
    }
}
