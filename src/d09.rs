use anyhow::Context;

pub fn part_01() -> anyhow::Result<u32> {
    let inputs = inputs()?;

    Ok(solve_01(inputs))
}

fn solve_01(inputs: Vec<Vec<u32>>) -> u32 {
    let max_y = inputs.len();
    let max_x = inputs[0].len();

    let mut sum = 0;
    for iy in 0..max_y {
        for ix in 0..max_x {
            sum += calc_neighbours(ix, iy, &inputs);
        }
    }
    sum
}

fn calc_neighbours(ix: usize, iy: usize, inputs: &Vec<Vec<u32>>) -> u32 {
    let nnxs = get_neighbour_ixs(ix, iy, inputs);

    let cur = inputs[iy][ix];
    for (nx, ny) in nnxs {
        if cur >= inputs[ny][nx] {
            return 0;
        }
    }
    return 1 + cur;
}

fn get_neighbour_ixs(ix: usize, iy: usize, inputs: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let max_y = inputs.len();
    let max_x = inputs[0].len();

    let mut nnxs = vec![];
    if ix > 0 {
        nnxs.push((ix - 1, iy));
    }
    if iy > 0 {
        nnxs.push((ix, iy - 1));
    }
    if ix < max_x - 1 {
        nnxs.push((ix + 1, iy));
    }
    if iy < max_y - 1 {
        nnxs.push((ix, iy + 1))
    }
    nnxs
}

pub fn part_02() -> anyhow::Result<usize> {
    let mut inputs = inputs()?;

    let max_y = inputs.len();
    let max_x = inputs[0].len();

    let mut res = vec![];
    for iy in 0..max_y {
        for ix in 0..max_x {
            let one = basin_calc(ix, iy, &mut inputs);
            if one > 0 {
                res.push(one);
            }
        }
    }

    res.sort();
    Ok(res.iter().rev().take(3).product())
}

fn basin_calc(ix: usize, iy: usize, inputs: &mut Vec<Vec<u32>>) -> usize {
    let mut num_filled = 0;
    let mut queue: std::collections::VecDeque<(usize, usize)> = [(ix, iy)].into();
    while let Some((x, y)) = queue.pop_front() {
        if inputs[y][x] == 9 {
            continue;
        }

        inputs[y][x] = 9;
        num_filled += 1;
        for index in get_neighbour_ixs(x, y, inputs) {
            queue.push_front(index);
        }
    }
    num_filled
}

fn inputs() -> anyhow::Result<Vec<Vec<u32>>> {
    let input_string =
        std::fs::read_to_string("inputs/09_input.txt").context("Error while reading input")?;

    Ok(parse(&input_string))
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut res = vec![];
    for line in input.lines() {
        let row = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        res.push(row)
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input_01() {
        let inputs = parse(
            "1299943210
        3987894921
        9856789892
        8767896789
        0899965670",
        );

        assert_eq!(solve_01(inputs), 17);
    }
}
