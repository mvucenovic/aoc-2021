use anyhow::Context;
use std::collections::HashMap;

pub fn part_01() -> anyhow::Result<u64> {
    let positions = inputs()?;

    let res = solve_01(&positions);

    Ok(res.into_iter().min().unwrap())
}

fn solve_01(positions: &[u64]) -> Vec<u64> {
    let mut map = HashMap::new();

    for position in positions.iter() {
        let val = map.entry(*position).or_insert(0u64);
        *val += 1;
    }

    let mut left_sum = 0u64;
    let mut left_count = 0u64;

    let mut right_sum: u64 = positions.iter().sum();
    let mut right_count = positions.len() as u64;

    let mut sums = vec![];

    let max_pos = positions.iter().max().unwrap();
    for i in 0..=*max_pos {
        sums.push(left_sum + right_sum);

        let cur_count = *map.get(&i).unwrap_or(&0);
        left_count += cur_count;
        left_sum += left_count;

        right_count -= cur_count;
        right_sum -= right_count;
    }

    sums
}

pub fn part_02() -> anyhow::Result<i64> {
    let positions = inputs()?;

    Ok(*solve_02(&positions).iter().min().unwrap())
}

fn calc_fuel_02(ix: u64, map: &HashMap<u64, u64>) -> i64 {
    let mut sum = 0;
    for (&k, &v) in map {
        let distance = (ix as i64 - k as i64).abs();
        let fuel_cost = distance * (distance + 1) / 2;
        sum += fuel_cost * v as i64;
    }
    sum
}

fn solve_02(positions: &[u64]) -> Vec<i64> {
    let mut map = HashMap::new();

    for position in positions.iter() {
        let val = map.entry(*position).or_insert(0u64);
        *val += 1;
    }

    let max_pos = positions.iter().max().unwrap();
    let mut sums = vec![];
    for i in 0..=*max_pos {
        sums.push(calc_fuel_02(i, &map));
    }

    sums
}

fn inputs() -> anyhow::Result<Vec<u64>> {
    let input_string =
        std::fs::read_to_string("inputs/07_input.txt").context("Error while reading input")?;

    Ok(input_string
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<_, _>>()?)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inputs_01() {
        let inputs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let res = solve_01(&inputs);
        assert_eq!(res[1], 41);
        assert_eq!(res[3], 39);
        assert_eq!(res[10], 71);
    }

    #[test]
    fn test_inputs_02() {
        let inputs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let res = solve_02(&inputs);
        assert_eq!(res[5], 168);
    }
}
