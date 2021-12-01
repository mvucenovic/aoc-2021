use anyhow::Context;
use itertools::Itertools;

fn inputs() -> anyhow::Result<Vec<usize>> {
    let input_string =
        std::fs::read_to_string("inputs/01_input.txt").context("Error while reading input")?;

    Ok(input_string
        .split('\n')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>())
}

pub fn part_01() -> anyhow::Result<usize> {
    let depths = inputs()?;

    Ok(itertools::fold(
        depths.iter().tuple_windows(),
        0,
        |acc, (d1, d2)| {
            if d1 < d2 {
                acc + 1
            } else {
                acc
            }
        },
    ))
}

pub fn part_02() -> anyhow::Result<usize> {
    let depths = inputs()?;

    Ok(itertools::fold(
        depths.iter().tuple_windows().tuple_windows(),
        0,
        |acc, ((a1, a2, a3), (b1, b2, b3))| {
            if a1 + a2 + a3 < b1 + b2 + b3 {
                acc + 1
            } else {
                acc
            }
        },
    ))
}
