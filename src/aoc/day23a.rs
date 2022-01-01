use crate::aoc::file;
use std::cmp::min;
use std::collections::HashSet;
use std::path::Path;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Board {
    board: Vec<Vec<char>>,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

struct Move {
    from: Pos,
    to: Pos,
    cost: u32,
}

const HOME_A: usize = 3;
const HOME_B: usize = 5;
const HOME_C: usize = 7;
const HOME_D: usize = 9;
const HOME_Y1: usize = 2;
const HOME_Y2: usize = 3;

impl Board {
    fn get_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        for y in 1..3 {
            let line = &self.board[y];
            for x in 1..line.len() - 1 {
                let c = line[x];
                match c {
                    'A' => self.add_moves(&mut moves, Pos { x: x, y: y }, 1, HOME_A),
                    'B' => self.add_moves(&mut moves, Pos { x: x, y: y }, 10, 5),
                    'C' => self.add_moves(&mut moves, Pos { x: x, y: y }, 100, 7),
                    'D' => self.add_moves(&mut moves, Pos { x: x, y: y }, 1000, 9),
                    _ => {}
                }
            }
        }
        moves
    }

    fn add_moves(&self, moves: &mut Vec<Move>, pos: Pos, cost: u32, homex: usize) {
        let neighbours = pos.neighbours();
        for n in neighbours {
            if self.is_free(&n) && !self.is_foreign_home_move(&pos, &n, homex) {
                moves.push(Move {
                    from: pos.clone(),
                    to: n,
                    cost: cost,
                });
            }
        }
    }

    fn is_free(&self, pos: &Pos) -> bool {
        self.board[pos.y][pos.x] == '.'
    }
    fn is_foreign_home_move(&self, from: &Pos, to: &Pos, homex: usize) -> bool {
        to.y > from.y && to.x != homex
    }

    fn is_complete(&self) -> bool {
        /*        self.board[HOME_Y1][HOME_A] == 'A'
        && self.board[HOME_Y2][HOME_A] == 'A'
        && self.board[HOME_Y1][HOME_B] == 'B'
        && self.board[HOME_Y2][HOME_B] == 'B'
        && self.board[HOME_Y1][HOME_C] == 'C'
        && self.board[HOME_Y2][HOME_C] == 'C'
        && */
        self.board[HOME_Y1][HOME_D] == 'D' && self.board[HOME_Y2][HOME_D] == 'D'
    }

    fn moove(&self, m: &Move) -> Board {
        let mut b = self.clone();
        b.board[m.to.y][m.to.x] = self.board[m.from.y][m.from.x];
        b.board[m.from.y][m.from.x] = '.';
        b
    }
}

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        vec![
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

fn solve(b: &Board) -> u32 {
    let mut path: HashSet<Board> = HashSet::new();
    solve_1(&b, &mut path, 0, 20000)
}

fn solve_1(b: &Board, path: &mut HashSet<Board>, cost: u32, mincost: u32) -> u32 {
    if b.is_complete() {
        cost
    } else {
        let mut movs = b.get_moves();
        if movs.is_empty() {
            u32::MAX
        } else {
            let mut mincost_here = mincost;
            let mut cost_here = u32::MAX;
            movs.sort_by(|a, b| b.cost.cmp(&a.cost));
            for m in movs {
                if m.cost + cost < mincost {
                    let next = b.moove(&m);
                    if path.insert(next.clone()) {
                        let c = solve_1(&next, path, cost + m.cost, mincost_here);
                        cost_here = min(cost_here, c);
                        mincost_here = min(mincost_here, c);
                        path.remove(&next);
                    } else {
                        println!("duplicate");
                    }
                }
            }
            cost_here
        }
    }
}

fn read_input(f: &Path) -> Board {
    let lines = file::read_lines(f).unwrap();
    let mut b = Vec::new();
    for line in lines {
        let line = line.unwrap();
        b.push(line.chars().collect());
    }
    Board { board: b }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn result() {
        let b = read_input(&file::input("input23.txt"));
        let result = solve(&b);
        assert_eq!(result, 1);

        //assert_eq!(result, 900099);
    }
}
