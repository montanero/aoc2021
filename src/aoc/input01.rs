use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub fn split_line(line: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (\d+)\s*
            "
        )
        .unwrap();
    }

    let mut v = Vec::new();

    for cap in RE.captures_iter(line) {
        v.push(FromStr::from_str(&cap[1]).unwrap())
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let v = split_line("");
        assert_eq!(v.len(), 0)
    }

    #[test]
    fn one_number() {
        let v = split_line("23");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], 23);
    }

    #[test]
    fn one_number_with_blanks() {
        let v = split_line("  245  ");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], 245);
    }

    #[test]
    fn two_numbers () {
        let v = split_line(" 1 2");
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
    }

    #[test]
    fn two_numbers_with_spaces () {
        let v = split_line("   3 42  ");
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 3);
        assert_eq!(v[1], 42);
    }
}