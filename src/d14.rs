use anyhow::Context;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_01() -> anyhow::Result<u64> {
    let (starting_polimer, map) = inputs()?;

    Ok(solve_01(10, starting_polimer, map))
}

fn solve_01(steps: usize, starting_polimer: String, map: HashMap<(char, char), char>) -> u64 {
    let mut results = HashMap::new();
    starting_polimer
        .chars()
        .for_each(|c| *results.entry(c).or_insert(0) += 1);

    for (c0, c1) in starting_polimer.chars().tuple_windows() {
        go_deeper(steps, c0, c1, &map, &mut results);
    }

    calc_res(&results)
}

fn go_deeper(
    steps: usize,
    c0: char,
    c1: char,
    mappings: &HashMap<(char, char), char>,
    results: &mut HashMap<char, u64>,
) {
    if steps == 0 {
        return;
    }

    let c2 = *mappings.get(&(c0, c1)).unwrap();
    *results.entry(c2).or_insert(0) += 1;

    go_deeper(steps - 1, c0, c2, mappings, results);
    go_deeper(steps - 1, c2, c1, mappings, results);
}

pub fn part_02() -> anyhow::Result<u64> {
    let (starting_polimer, map) = inputs()?;

    let mut results = HashMap::new();
    starting_polimer
        .chars()
        .for_each(|c| *results.entry(c).or_insert(0) += 1);

    let mut pairs = HashMap::new();
    for (c0, c1) in starting_polimer.chars().tuple_windows() {
        *pairs.entry((c0, c1)).or_insert(0) += 1;
    }

    for _i in 0..40 {
        let mut after = HashMap::new();
        for (&from, &cnt) in pairs.iter() {
            let from = from.clone();
            let to = map.get(&from).unwrap().clone();
            *after.entry((from.0, to)).or_insert(0) += cnt;
            *after.entry((to, from.1)).or_insert(0) += cnt;
            *results.entry(to).or_insert(0) += cnt;
        }
        pairs = after;
    }

    Ok(calc_res(&results))
}

fn calc_res(results: &HashMap<char, u64>) -> u64 {
    let most_common = results.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    let least_common = results.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();

    *most_common.1 - *least_common.1
}

fn inputs() -> anyhow::Result<(String, HashMap<(char, char), char>)> {
    let input_string =
        std::fs::read_to_string("inputs/14_input.txt").context("Error while reading input")?;

    let mut split = input_string.split("\n\n");

    let starting_polimer = split.next().context("Bad input")?.trim().to_owned();

    let mappings = split.next().context("Bad input")?;
    let mut map = HashMap::new();
    for line in mappings.lines() {
        let mut split = line.trim().split(" -> ");
        let mut from = split.next().context("Bad input ()->")?.chars();
        let to = split.next().context("bad input ->()")?;

        let (c0, c1) = (
            from.next().context("bad input (c0,_)")?,
            from.next().context("bad input (_,c1)")?,
        );
        let to = to.chars().next().context("bad input (to)")?;
        map.insert((c0, c1), to);
    }

    Ok((starting_polimer, map))
}
