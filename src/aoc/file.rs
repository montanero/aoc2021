use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    Ok(lines)
}

pub fn input(filename: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/input");
    d.push(filename);
    d
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use std::path::PathBuf;

    use super::*;

    fn test_file(filename: &str) -> PathBuf {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        d.push(filename);
        d
    }

    #[test]
    fn read_empty_file() {
        let bla: Lines<BufReader<File>> = read_lines(&test_file("empty.txt")).unwrap();
        let mut count = 0;
        for _line in bla {
            count = count + 1;
        }
        assert_eq!(count, 0)
    }

    #[test]
    fn read_non_empty_file() {
        let bla: Lines<BufReader<File>> = read_lines(&test_file("twolines.txt")).unwrap();
        let mut count = 0;
        for line in bla {
            count = count + 1;
            let line = line.unwrap();
            assert_eq!(line, String::from("line") + &count.to_string())
        }
        assert_eq!(count, 2)
    }
}
