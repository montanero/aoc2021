use std::cmp::min;
use std::path::Path;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input15.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let levels = read_file(f);
    let mut solver = Solver::create(levels);
    solver.calculate();
    solver.distance[solver.size_y - 1][solver.size_x - 1]
}

struct Solver {
    levels: Vec<Vec<i8>>,
    distance: Vec<Vec<u32>>,
    size_x: usize,
    size_y: usize,
}

impl Solver {
    fn create(levels: Vec<Vec<i8>>) -> Solver {
        let size_x = levels[0].len();
        let size_y = levels.len();
        let mut distance = vec![];
        distance.push(Self::create_initial_line(&levels, 0, 0));
        for y in 1..size_y {
            distance.push(Self::create_initial_line(
                &levels,
                y,
                distance[y - 1][0] + levels[y][0] as u32,
            ));
        }
        Solver {
            levels: levels,
            distance: distance,
            size_x: size_x,
            size_y: size_y,
        }
    }

    fn create_initial_line(levels: &Vec<Vec<i8>>, y: usize, left_value: u32) -> Vec<u32> {
        let mut distline = vec![];
        distline.push(left_value);
        for x in 1..levels[y].len() {
            distline.push(distline.last().unwrap() + levels[y][x] as u32)
        }
        distline
    }

    fn calculate(&mut self) {
        loop {
            let mut changed = false;
            for y in 0..self.size_y {
                for x in 0..self.size_x {
                    changed |= self.fix(x, y)
                }
            }
            if !changed {
                break;
            }
        }
    }

    fn fix(&mut self, x: usize, y: usize) -> bool {
        //let dirs: Vec<Dir> = vec![Dir::R, Dir::D, Dir::U, Dir::L];
        let mut min_dist = self.distance[y][x];
        let level = self.levels[y][x];
        if x > 0 {
            min_dist = min(min_dist, level as u32 + self.distance[y][x - 1])
        }
        if y > 0 {
            min_dist = min(min_dist, level as u32 + self.distance[y - 1][x])
        }
        if y < self.size_y - 1 {
            min_dist = min(min_dist, level as u32 + self.distance[y + 1][x])
        }
        if x < self.size_x - 1 {
            min_dist = min(min_dist, level as u32 + self.distance[y][x + 1])
        }
        if min_dist < self.distance[y][x] {
            self.distance[y][x] = min_dist;
            true
        } else {
            false
        }
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
        assert_eq!(result, 393);
    }

    #[test]
    fn result_sample() {
        let result = solve_file(&file::input("input15-sample.txt"));
        println!("result : {}", result);
        assert_eq!(result, 40);
    }
}
