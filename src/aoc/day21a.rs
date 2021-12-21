use std::path::Path;

use crate::aoc::file;

struct Player {
    pos: u32,
    score: u32,
}

struct Die {
    next: u32,
    rolls: u32,
}

struct Game {
    p1: Player,
    p2: Player,
    die: Die,
}

impl Die {
    fn roll(&mut self) -> u32 {
        let res = self.next;
        self.next = 1 + (res % 1000);
        self.rolls += 1;
        res
    }
}

impl Player {
    fn mov(&mut self, dist: u32) {
        let n = (self.pos + dist - 1) % 10 + 1;
        self.pos = n;
        self.score += n;
    }
}

impl Game {
    fn create(pos1: u32, pos2: u32) -> Game {
        Game {
            p1: Player {
                pos: pos1,
                score: 0,
            },
            p2: Player {
                pos: pos2,
                score: 0,
            },
            die: Die { next: 1, rolls: 0 },
        }
    }

    fn mov(&mut self) -> u32 {
        let res: u32;
        loop {
            let dist1 = self.die.roll() + self.die.roll() + self.die.roll();
            self.p1.mov(dist1);
            if (self.p1.score >= 1000) {
                res = self.p2.score * self.die.rolls;
                break;
            }
            let dist2 = self.die.roll() + self.die.roll() + self.die.roll();
            self.p2.mov(dist2);
            if (self.p2.score >= 1000) {
                res = self.p1.score * self.die.rolls;
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut game = Game::create(10, 6);
        let result = game.mov();
        assert_eq!(result, 900099);
    }
}
