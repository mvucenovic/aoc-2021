use std::collections::HashMap;

use anyhow::Context;
use std::collections::VecDeque;

fn inputs() -> anyhow::Result<HashMap<String, Vec<String>>> {
    let input_string =
        std::fs::read_to_string("inputs/12_input.txt").context("Error while reading input")?;

    Ok(parse(&input_string))
}

fn parse(s: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let mut split = line.trim().split("-");

        let from1 = split.next().unwrap().to_owned();
        let to1 = split.next().unwrap().to_owned();

        let from2 = to1.clone();
        let to2 = from1.clone();

        map.entry(from1).or_insert(vec![]).push(to1);
        map.entry(from2).or_insert(vec![]).push(to2);
    }
    map
}

pub fn part_01() -> anyhow::Result<i32> {
    let inputs = inputs()?;

    Ok(solveR_01(&inputs, "start".to_owned()))
}

fn recurse_01(
    map: &HashMap<String, Vec<String>>,
    current: String,
    curr_path: &mut VecDeque<String>,
    mut visited_twice: bool,
) -> i32 {
    if current == "end" {
        return 1;
    }

    if is_small_cave(&current) && curr_path.contains(&current) {
        if visited_twice || current == "start" {
            return 0;
        }
        visited_twice = true;
    }

    let mut res = 0;
    curr_path.push_back(current.clone());
    let neighbours = map.get(&current).unwrap();
    for neighbour in neighbours {
        res += recurse_01(map, neighbour.clone(), curr_path, visited_twice);
    }
    curr_path.pop_back();

    res
}

fn solveR_01(map: &HashMap<String, Vec<String>>, current: String) -> i32 {
    let mut curr_path = VecDeque::new();

    let res = recurse_01(map, current, &mut curr_path, true);
    res
}

fn solveR_02(map: &HashMap<String, Vec<String>>, current: String) -> i32 {
    let mut curr_path = VecDeque::new();

    let res = recurse_01(map, current, &mut curr_path, false);
    res
}

fn is_small_cave(s: &String) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

pub fn part_02() -> anyhow::Result<i32> {
    let inputs = inputs()?;

    Ok(solveR_02(&inputs, "start".to_owned()))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_01_01() {
        let map = parse(
            "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end",
        );
        let res = solveR_01(&map, "start".to_owned());

        assert_eq!(res, 10);
    }

    #[test]
    fn test_part_01_02() {
        let map = parse(
            "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc",
        );
        let res = solveR_01(&map, "start".to_owned());

        assert_eq!(res, 19);
    }

    #[test]
    fn test_part_01_03() {
        let map = parse(
            "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW",
        );

        let res = solveR_01(&map, "start".to_owned());

        assert_eq!(res, 226);
    }
}
