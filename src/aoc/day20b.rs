use std::path::Path;

use crate::aoc::file;

impl Data {
    fn turn(&mut self, border: char) {
        self.enlarge(border);
        self.enlarge(border);
        let mut f2 = vec![];
        for y in 1..self.field.len() - 1 {
            let mut line = vec![];
            for x in 1..self.field[0].len() - 1 {
                let code = self.get_index(x, y);
                line.push(self.decode_code(code))
            }
            f2.push(line)
        }
        self.field = f2;
    }
    fn enlarge(&mut self, border: char) {
        let len = self.field[0].len();
        self.field.insert(0, vec![border; len]);
        self.field.push(vec![border; len]);
        for x in &mut self.field {
            x.insert(0, border);
            x.push(border);
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (self.get_bit(x - 1, y - 1) << 8)
            + (self.get_bit(x, y - 1) << 7)
            + (self.get_bit(x + 1, y - 1) << 6)
            + (self.get_bit(x - 1, y) << 5)
            + (self.get_bit(x, y) << 4)
            + (self.get_bit(x + 1, y) << 3)
            + (self.get_bit(x - 1, y + 1) << 2)
            + (self.get_bit(x, y + 1) << 1)
            + (self.get_bit(x + 1, y + 1))
    }

    fn get_bit(&self, x: usize, y: usize) -> usize {
        let c = self.field[y][x];
        match c {
            '#' => 1,
            _ => 0,
        }
    }

    fn decode_code(&self, idx: usize) -> char {
        self.decode.as_bytes()[idx] as char
    }

    fn count_hashes(self) -> usize {
        self.field
            .iter()
            .map(|l| l.iter().filter(|c| **c == '#').count())
            .sum()
    }
}

fn read_file(f: &Path) -> Data {
    let mut lines = file::read_lines(f).unwrap();
    let decode = lines.next().unwrap().unwrap();
    let _empty = lines.next();
    let mut vec = vec![];

    for line in lines {
        let l: Vec<char> = line.unwrap().chars().collect();
        vec.push(l)
    }

    Data {
        decode: decode,
        field: vec,
    }
}

struct Data {
    decode: String,
    field: Vec<Vec<char>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut inp = read_file(&file::input("input20.txt"));
        for _ in 0..25 {
            inp.turn('.');
            inp.turn('#');
        }
        let result = inp.count_hashes();
        println!("result : {}", result);
        assert_eq!(result, 5395);
    }

    #[test]
    fn sample() {
        let mut inp = read_file(&file::input("input20-sample.txt"));
        for _ in 0..25 {
            inp.turn('.');
            inp.turn('.');
        }
        let result = inp.count_hashes();
        println!("result : {}", result);
        assert_eq!(result, 3351);
    }
}
