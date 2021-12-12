use anyhow::Context;
use itertools::Itertools;

use std::collections::VecDeque;

fn inputs() -> anyhow::Result<Vec<Vec<u32>>> {
    let input_string =
        std::fs::read_to_string("inputs/11_input.txt").context("Error while reading input")?;

    Ok(parse(&input_string))
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|s| s.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<_>>()
}

pub fn part_01() -> anyhow::Result<u32> {
    let mut inputs = inputs()?;

    let mut res = 0;
    for _s in 0..100 {
        res += step(&mut inputs);
    }

    Ok(res)
}

fn step(inputs: &mut Vec<Vec<u32>>) -> u32 {
    let mut flash_cnt = 0;

    let mut flashing = VecDeque::new();

    itertools::iproduct!(0..10, 0..10).for_each(|(x, y)| {
        inputs[y][x] += 1;
        if inputs[y][x] == 10 {
            flash_cnt += 1;
            flashing.push_back((x, y));
        }
    });

    while !flashing.is_empty() {
        let (ix, iy) = flashing.pop_front().unwrap();
        let neighbours = neighbours(ix, iy);
        for (nx, ny) in neighbours {
            inputs[ny][nx] += 1;
            if inputs[ny][nx] == 10 {
                flash_cnt += 1;
                flashing.push_back((nx, ny));
            }
        }
    }

    itertools::iproduct!(0..10, 0..10).for_each(|(x, y)| {
        if inputs[y][x] > 9 {
            inputs[y][x] = 0;
        }
    });
    flash_cnt
}

fn neighbours(ix: usize, iy: usize) -> Vec<(usize, usize)> {
    let ix = ix as isize;
    let iy = iy as isize;

    itertools::iproduct!(-1..=1, -1..=1)
        .filter(|(dx, dy)| *dx != 0 || *dy != 0)
        .map(|(dx, dy)| (ix + dx, iy + dy))
        .filter(|&(x, y)| x >= 0 && x < 10 && y >= 0 && y < 10)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

pub fn part_02() -> anyhow::Result<u32> {
    let mut inputs = inputs()?;

    let mut s = 0;
    while true {
        s += 1;
        step(&mut inputs);
        if inputs.iter().all(|row| row.iter().all(|el| *el == 0)) {
            return Ok(s);
        }
    }

    Ok(0)
}

fn pprint(n: &Vec<Vec<u32>>) {
    for iy in 0..10 {
        let line = n[iy].iter().map(|d| d.to_string()).join("");
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inputs_01() {
        let inputs = "6594254334
        3856965822
        6375667284
        7252447257
        7468496589
        5278635756
        3287952832
        7993992245
        5957959665
        6394862637";

        let mut inputs = parse(inputs);
        let res = step(&mut inputs);
        assert_eq!(res, 35);
        assert_eq!(step(&mut inputs), 45);
        assert_eq!(step(&mut inputs), 16);
        assert_eq!(step(&mut inputs), 8);
    }

    #[test]
    fn test_neighbours() {
        let ns = neighbours(2, 0);
        assert_eq!(ns.len(), 5);
    }
}
