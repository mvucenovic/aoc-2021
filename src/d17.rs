pub fn part_01() -> anyhow::Result<i32> {
    let y1: i32 = -129;
    let _y2: i32 = -70;

    // if Y speed is -128 by the time Y position is 0 again (on the way down), probe will reach highest possible Y.
    // reason is the fact that that is the highest possible Y speed that actually hists the rectangle, with the way it
    // slows down one by one.

    // So we just need to find out, if the speed on the way down is -128, what was that Y position.
    // Since the speed is always incrementing in steps of one from the highest possible Y position,
    // speed is -1, -2, -3, -4, -5, -6 ... -128 (and finally -129)

    // so the sum all of those speeds is actually the solution, since the Y speed at highest point was 0,
    // and once it reached the position (x_whatever, 0), it will be -128
    // (so that the next step is on -129 and fits right on the edge of my input).

    // This is just simple triangular number series. 1..128

    let y_speed_at_zero = y1.abs() - 1;
    Ok(y_speed_at_zero * (y_speed_at_zero + 1) / 2)
}

pub fn part_02() -> anyhow::Result<usize> {
    let xs = (150, 171);
    let ys = (-129i32, -70i32);

    let (_, x2) = xs;
    let (y1, _) = ys;

    let mut results = vec![];

    for vx in 1..=x2 {
        // our X speed cannot be over x2, since we would imediatly step over
        for vy in y1..=y1.abs() {
            // Y speed cannot be over y1, since we would imediatly step over

            let mut current_pos = (0, 0);
            let mut current_v = (vx, vy);

            while should_we_continue(current_pos, ys) {
                if is_inside(current_pos, xs, ys) {
                    results.push((vx, vy));
                    break;
                }
                let (new_pos, new_v) = step(current_pos, current_v);
                current_pos = new_pos;
                current_v = new_v;
            }
        }
    }

    Ok(results.len())
}

fn should_we_continue(current_pos: (i32, i32), ys: (i32, i32)) -> bool {
    let (_, y) = current_pos;
    // we missed the mark on Y axis
    if y < ys.0 {
        // we will never go up again
        return false;
    }
    true
}

fn step(current_pos: (i32, i32), current_v: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let (vx, vy) = current_v;
    let new_pos = (current_pos.0 + vx, current_pos.1 + vy);

    let new_vx = if vx == 0 {
        0
    } else if vx < 0 {
        vx + 1
    } else {
        vx - 1
    };

    let new_vy = vy - 1;

    (new_pos, (new_vx, new_vy))
}

fn is_inside(current_pos: (i32, i32), xs: (i32, i32), ys: (i32, i32)) -> bool {
    if current_pos.0 >= xs.0
        && current_pos.0 <= xs.1
        && current_pos.1 >= ys.0
        && current_pos.1 <= ys.1
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_inside() {
        assert_eq!(should_we_continue((29, -5), (-10, -5)), true);
        assert_eq!(is_inside((29, -5), (20, 30), (-10, -5)), true);
    }
}
