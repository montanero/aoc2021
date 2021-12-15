use std::cmp::min;
use std::collections::HashSet;
use std::path::Path;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input15.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let levels = read_file(f);
    let mut solver = Solver::create(levels);
    solver.calculate();
    solver.distance[0][0].unwrap() - solver.levels[0][0] as u32
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Pos {
    x: usize,
    y: usize,
}
#[derive(Clone)]
enum Dir {
    R,
    D,
    L,
    U,
}

impl Pos {
    fn do_move(&self, d: &Dir) -> Pos {
        match d {
            Dir::R => Pos {
                x: self.x + 1,
                y: self.y,
            },
            Dir::D => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::L => Pos {
                x: if self.x > 0 { self.x - 1 } else { usize::MAX },
                y: self.y,
            },
            Dir::U => Pos {
                x: self.x,
                y: if self.y > 0 { self.y - 1 } else { usize::MAX },
            },
        }
    }
}

struct Solver {
    levels: Vec<Vec<i8>>,
    distance: Vec<Vec<Option<u32>>>,
    size: Pos,
}

impl Solver {
    fn create(levels: Vec<Vec<i8>>) -> Solver {
        let size = Pos {
            x: levels[0].len(),
            y: levels.len(),
        };
        let distance = vec![vec![Option::None; size.x]; size.y];
        Solver {
            levels: levels,
            distance: distance,
            size: size,
        }
    }

    fn calculate(&mut self) {
        for d in 0..self.size.x {
            let mut y = self.size.y;
            for x in self.size.x - d - 1..self.size.x {
                y -= 1;
                let pos = Pos { x, y };
                let dist = self.calculate_distance(&pos);
                self.distance[y][x] = Option::Some(dist);
            }
        }
    }

    fn calculate_distance(&self, now: &Pos) -> u32 {
        let mut visited: HashSet<Pos> = HashSet::new();
        self.get_path_from(now, 0, u32::MAX, &mut visited)
    }

    fn get_path_from(
        &self,
        now: &Pos,
        level: u32,
        bestlevel: u32,
        visited: &mut HashSet<Pos>,
    ) -> u32 {
        if !self.on_board(now) {
            u32::MAX
        } else {
            match self.distance[now.y][now.x] {
                Some(d) => level + d,
                None => {
                    if visited.contains(now) {
                        u32::MAX
                    } else {
                        let newlevel = level + self.levels[now.y][now.x] as u32;
                        if newlevel + self.dist_to_endpos(now) as u32 >= bestlevel {
                            u32::MAX
                        } else if self.on_endpos(now) {
                            newlevel
                        } else {
                            visited.insert((*now).clone());
                            let r = self.try_directions(now, newlevel, bestlevel, visited);
                            visited.remove(&now.clone());
                            r
                        }
                    }
                }
            }
        }
    }

    fn try_directions(
        &self,
        now: &Pos,
        level: u32,
        bestlevel: u32,
        visited: &mut HashSet<Pos>,
    ) -> u32 {
        let dirs: Vec<Dir> = vec![Dir::R, Dir::D, Dir::U, Dir::L];
        let mut best = bestlevel;
        for d in dirs {
            let lr = self.try_directions_sub(now, level, best, visited, d);
            best = min(best, lr);
        }
        best
    }

    fn try_directions_sub(
        &self,
        now: &Pos,
        level: u32,
        bestlevel: u32,
        visited: &mut HashSet<Pos>,
        d: Dir,
    ) -> u32 {
        let p = now.do_move(&d);
        self.get_path_from(&p, level, bestlevel, visited)
    }

    fn on_board(&self, pos: &Pos) -> bool {
        pos.x < self.size.x && pos.y < self.size.y
    }
    fn on_endpos(&self, pos: &Pos) -> bool {
        pos.x == self.size.x - 1 && pos.y == self.size.y - 1
    }
    fn dist_to_endpos(&self, pos: &Pos) -> usize {
        (self.size.x - 1) - pos.x + (self.size.y - 1) - pos.y
    }
}

fn read_file(p0: &Path) -> Vec<Vec<i8>> {
    let input = file::read_lines(p0).unwrap();
    input.map(|l| parse_line(&l.expect("fail"))).collect()
}

fn parse_line(line: &str) -> Vec<i8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 489);
    }

    #[test]
    fn result_sample() {
        let result = solve_file(&file::input("input15-sample.txt"));
        println!("result : {}", result);
        assert_eq!(result, 40);
    }
}
