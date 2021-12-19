use std::collections::HashSet;

fn get_velocities_x(xl: i32, xr: i32) -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    for vel in (1..xr + 1).rev() {
        let mut v = vel;
        let mut x = 0;
        'i: for i in 1..1000 {
            x += v;
            v -= x.signum();
            if x >= xl && x <= xr {
                ret.push((i, vel))
            } else if x > xr {
                break 'i;
            }
        }
    }
    ret
}

fn get_velocities_y(yb: i32, yt: i32) -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    for vel in yb..1000 {
        let mut v = vel;
        let mut y = 0;
        'i: for i in 1..1000 {
            y += v;
            v -= 1;
            if y >= yb && y <= yt {
                ret.push((i, vel))
            } else if y < yb {
                break 'i;
            }
        }
    }
    ret
}

fn result(x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
    let vx = get_velocities_x(x0, x1);
    let vy = get_velocities_y(y0, y1);
    let mut vecs: HashSet<(i32, i32)> = HashSet::new();
    for (xi, xv) in &vx {
        for (yi, yv) in &vy {
            if xi == yi {
                vecs.insert((*xv, *yv));
            }
        }
    }
    for (x, y) in &vecs {
        println!("{},{}", x, y)
    }
    vecs.len() as i32
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
        assert_eq!(res, 112)
    }
}
