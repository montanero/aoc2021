use std::ops::Mul;
use std::path::Path;

use lazy_static::lazy_static;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input19.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let trees = read_file(f);
    0
}

fn read_file(f: &Path) -> u32 {
    let lines = file::read_lines(f).unwrap();
    for line in lines {}
    0
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

struct Matrix {
    m: Vec<Vec<i32>>,
}

lazy_static! {
    static ref IDENTITY: Matrix = Matrix {
                m: vec![            vec![1, 0, 0], //
            vec![0, 1, 0], //
            vec![0, 0, 1]]};

static ref ROT_X: Matrix = Matrix {
    m: vec![
        //
        vec![1, 0, 0],  //
        vec![0, 0, -1], //
        vec![0, 1, 0],
    ]};

static ref ROT_Y: Matrix = Matrix {
    m: vec![
        //
        vec![0, 0, 1], //
        vec![0, 1, 0], //
        vec![-1, 0, 0],
    ],
};

static ref ROT_Z: Matrix = Matrix {
    m: vec![
        //
        vec![0, -1, 0], //
        vec![1, 0, 0],  //
        vec![0, 0, 1],
    ],
};

static ref ALL_ROTS: Vec<&'static Matrix> = vec![
    //
    &IDENTITY,             //
    &ROT_X,                 //
//    (&ROT_X) * &ROT_X,         //
//    ROT_X * ROT_X *ROT_X, //
];
    }

impl Matrix {
    fn lines(&self) -> usize {
        self.m.len()
    }

    fn columns(&self) -> usize {
        self.m[0].len()
    }

    fn mult_pos(&self, pos: &Pos) -> Pos {
        let po = pos.to_matrix();
        let mul = self * &po;
        Pos::from(&mul)
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, o: &'b Matrix) -> Matrix {
        if self.columns() != o.lines() {
            panic!("incompatible");
        }
        let mut m = vec![];
        for li in 0..self.lines() {
            let line = &self.m[li];
            let mut nline = vec![];
            for oci in 0..o.columns() {
                let mut sum = 0;
                for k in 0..line.len() {
                    sum += line[k] * o.m[k][oci];
                }
                nline.push(sum)
            }
            m.push(nline);
        }
        Matrix { m: m }
    }
}

impl Pos {
    fn to_matrix(&self) -> Matrix {
        Matrix {
            m: vec![
                //
                vec![self.x], //
                vec![self.y], //
                vec![self.z],
            ],
        }
    }

    fn from(m: &Matrix) -> Pos {
        assert_eq!(m.lines(), 3);
        assert_eq!(m.columns(), 1);
        Pos {
            x: m.m[0][0],
            y: m.m[1][0],
            z: m.m[2][0],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 3665);
    }

    #[test]
    fn rotx() {
        let po = Pos { x: 1, y: 2, z: 3 };
        let result = ROT_X.mult_pos(&po);
        assert_eq!(result, Pos { x: 1, y: -3, z: 2 });
    }
    #[test]
    fn rotxx() {
        let po = Pos { x: 1, y: 2, z: 3 };
        let result = (&ROT_X as &Matrix * &ROT_X as &Matrix).mult_pos(&po);
        assert_eq!(result, Pos { x: 1, y: -2, z: -3 });
    }

    #[test]
    fn roty() {
        let po = Pos { x: 1, y: 2, z: 3 };
        let result = ROT_Y.mult_pos(&po);
        assert_eq!(result, Pos { x: 3, y: 2, z: -1 });
    }

    #[test]
    fn rotz() {
        let po = Pos { x: 1, y: 2, z: 3 };
        let result = ROT_Z.mult_pos(&po);
        assert_eq!(result, Pos { x: -2, y: 1, z: 3 });
    }
}
