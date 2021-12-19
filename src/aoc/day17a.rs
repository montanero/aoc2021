use std::cmp::max;

use itertools::Itertools;

fn get_velocities_y(yb: i32, yt: i32) -> Vec<(i32, i32, i32)> {
    let mut ret = Vec::new();
    for vel in yb..1000 {
        let mut v = vel;
        let mut y = 0;
        let mut maxy = y;
        'i: for i in 1..1000 {
            y += v;
            v -= 1;
            if y >= yb && y <= yt {
                ret.push((i, vel, maxy))
            } else if y < yb {
                break 'i;
            }
            maxy = max(y, maxy)
        }
    }
    ret
}

fn result(_x0: i32, _x1: i32, y0: i32, y1: i32) -> i32 {
    let vy = get_velocities_y(y0, y1);
    let bla: Vec<_> = vy.iter().sorted_by(|a, b| b.2.cmp(&a.2)).collect();
    let res = bla[0].2;
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn result1() {
        let res = result(241, 273, -97, -63);
        assert_eq!(res, 4656)
    }

    #[test]
    fn sample() {
        let res = result(20, 30, -10, -5);
        assert_eq!(res, 45)
    }
}
