use std::path::Path;

use crate::aoc::file;

pub(crate) fn solve() -> u64 {
    solve_file(&file::input("input10.txt"))
}

fn solve_file(f: &Path) -> u64 {
    let lines = file::read_lines(f).unwrap();
    let mut results: Vec<u64> = Vec::new();
    for line in lines {
        let r = parse(&line.unwrap());
        match r {
            Syntax::Incomplete(score) => results.push(score),
            _ => {}
        }
    }
    results.sort();
    let result: u64 = *results.get(results.len() / 2).unwrap();
    result
}

enum Syntax {
    Ok,
    Incomplete(u64),
    Corruped(i32),
}

fn parse(line: &str) -> Syntax {
    let mut chars: Vec<char> = line.chars().collect();
    let rc = parse_multi(&mut chars);
    match rc {
        Syntax::Ok => {
            if chars.is_empty() {
                rc
            } else {
                Syntax::Corruped(get_score(&chars.remove(0)))
            }
        }
        _ => rc,
    }
}

fn parse_multi(line: &mut Vec<char>) -> Syntax {
    let result: Syntax;
    loop {
        if line.is_empty() {
            result = Syntax::Ok;
            break;
        }
        let first = line.remove(0);
        if is_closing(&first) {
            line.insert(0, first);
            result = Syntax::Ok;
            break;
        } else {
            let rc = parse_multi(line);
            match rc {
                Syntax::Ok => {
                    if line.is_empty() {
                        result = Syntax::Incomplete(get_incomplete_score(&first));
                        break;
                    } else {
                        let last = line.remove(0);
                        if is_matching(&first, &last) {
                            // no break
                        } else {
                            result = Syntax::Corruped(get_score(&last));
                            break;
                        }
                    }
                }
                Syntax::Incomplete(score) => {
                    result = Syntax::Incomplete(score * 5 + get_incomplete_score(&first));
                    break;
                }
                _ => {
                    result = rc;
                    break;
                }
            }
        }
    }
    result
}

fn is_closing(last: &char) -> bool {
    match last {
        ')' | ']' | '}' | '>' => true,
        _ => false,
    }
}

fn get_score(last: &char) -> i32 {
    match last {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_incomplete_score(last: &char) -> u64 {
    match last {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn is_matching(first: &char, last: &char) -> bool {
    match *first {
        '<' => *last == '>',
        '(' => *last == ')',
        '[' => *last == ']',
        '{' => *last == '}',
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 3539961434);
    }
    #[test]
    fn sample() {
        let result = solve_file(&file::input("input10-sample.txt"));
        println!("result : {}", result);
        assert_eq!(result, 288957);
    }

    #[test]
    fn s0() {
        let result = parse("()");
        assert!(matches!(result, Syntax::Ok));
    }
    #[test]
    fn s1() {
        let result = parse("(>");
        assert!(matches!(result, Syntax::Corruped(_)));
    }
    #[test]
    fn s2() {
        let result = parse("(");
        assert!(matches!(result, Syntax::Incomplete(1)));
    }
    #[test]
    fn s3() {
        let result = parse("()()");
        assert!(matches!(result, Syntax::Ok));
    }
    #[test]
    fn s4() {
        let result = parse(")");
        assert!(matches!(result, Syntax::Corruped(_)));
    }
}
