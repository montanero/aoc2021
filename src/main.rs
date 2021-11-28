//use std::io::Read;


mod
learning_1 {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests
    {
        #[test]
        fn sum() {
            use crate::learning_1::add;
            assert_eq!(add(1, 2), 3)
        }
    }
}

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score {}", match score {
        None => String::from("none"),
        Some(v) => v.to_string()
    });

    use std::fs::File;
    use std::io::Read;

    let mut f = File::open("input01.txt").unwrap();
    let mut s = String::new();


    f.read_to_string(&mut s).unwrap();
}

mod aoc {
    mod file {
        use std::fs::File;
        use std::io;
        use std::io::BufRead;
        use std::path::Path;

        fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
            let file = File::open(filename)?;
            let lines = io::BufReader::new(file).lines();
            Ok(lines)
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
                    assert_eq!(line, String::from("line")+&count.to_string())
                }
                assert_eq!(count, 2)
            }
        }
    }
}
