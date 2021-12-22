use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
struct PlayerConfiguration {
    pos: u32,
    score: u32,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct GameConfiguration {
    p1: PlayerConfiguration,
    p2: PlayerConfiguration,
}

struct Game {
    configs: HashMap<PlayerConfiguration, u32>,
}

impl PlayerConfiguration {
    fn mov(&self, dist: u32) -> PlayerConfiguration {
        let n = (self.pos + dist - 1) % 10 + 1;
        PlayerConfiguration {
            pos: n,
            score: self.score + n,
        }
    }

    fn is_final(&self) -> bool {
        self.score >= 21
    }
}

impl GameConfiguration {
    fn is_final(&self) -> bool {
        self.p1.is_final() || self.p2.is_final()
    }

    fn winner(&self) -> u32 {
        if self.p1.is_final() {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::max;

    #[test]
    fn result() {
        let initial = GameConfiguration {
            p1: PlayerConfiguration { pos: 10, score: 0 },
            p2: PlayerConfiguration { pos: 6, score: 0 },
        };

        let mut config = HashMap::new();
        let mut p1wins = 0u64;
        let mut p2wins = 0u64;

        config.insert(initial, 1);

        for mve in 0..21 {
            let mut new_config = HashMap::new();
            for (key, val) in config {
                if !key.is_final() {
                    for d11 in 1..4 {
                        for d12 in 1..4 {
                            for d13 in 1..4 {
                                let d1 = d11 + d12 + d13;
                                let new1 = key.p1.mov(d1);
                                if new1.is_final() {
                                    let newgc = GameConfiguration {
                                        p1: new1,
                                        p2: key.p2.clone(),
                                    };
                                    let count_o = new_config.get(&newgc);
                                    let count = match count_o {
                                        None => val,
                                        Some(x) => x + val,
                                    };
                                    new_config.insert(newgc, count);
                                } else {
                                    for d21 in 1..4 {
                                        for d22 in 1..4 {
                                            for d23 in 1..4 {
                                                let d2 = d21 + d22 + d23;
                                                let new2 = key.p2.mov(d2);
                                                let newgc = GameConfiguration {
                                                    p1: new1.clone(),
                                                    p2: new2,
                                                };
                                                let count_o = new_config.get(&newgc);
                                                let count = match count_o {
                                                    None => val,
                                                    Some(x) => x + val,
                                                };
                                                new_config.insert(newgc, count);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for (k, v) in &new_config {
                if (k.is_final()) {
                    if (k.winner() == 1) {
                        p1wins += v;
                    } else {
                        p2wins += v;
                    }
                }
            }
            config = new_config;
        }

        assert_eq!(max(p1wins, p2wins), 306719685234774)

        //assert_eq!(result, 900099);
    }
}
