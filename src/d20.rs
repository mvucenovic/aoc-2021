use anyhow::Context;
use std::collections::HashMap;

pub fn part_01() -> anyhow::Result<usize> {
    let (index, char_set) = inputs()?;

    let res = solve_01(char_set, &index, 2);
    Ok(num_of_lighted(&res))
}

pub fn part_02() -> anyhow::Result<usize> {
    let (index, char_map) = inputs()?;

    let res = solve_01(char_map, &index, 50);
    Ok(num_of_lighted(&res))
}

fn solve_01(
    mut char_map: HashMap<(i32, i32), char>,
    index: &[char],
    iterations: usize,
) -> HashMap<(i32, i32), char> {
    for i in 0..iterations {
        let mut new_map = HashMap::new();
        for (c, _) in char_map.iter() {
            for c2 in DGTS.iter().map(|d| (c.0 + d.0, c.1 + d.1)) {
                if new_map.contains_key(&c2) {
                    continue;
                }
                let ch = if index[0] == '.' || i % 2 == 0 {
                    '.'
                } else {
                    '#'
                };
                let bin_code = build_number(c2, &char_map, ch);
                new_map.insert(c2, index[bin_code]);
            }
        }
        char_map = new_map;
    }
    char_map
}

fn num_of_lighted(m: &HashMap<(i32, i32), char>) -> usize {
    m.iter().filter(|&(_, v)| *v == '#').count()
}

//-1,-1 -1,0 -1,1
//0,-1 0,0 0,1
//1,-1 1,0, 1,1
const DGTS: [(i32, i32); 9] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, 0),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn build_number(
    central_coor: (i32, i32),
    char_map: &HashMap<(i32, i32), char>,
    unknown: char,
) -> usize {
    let mut result = 0;

    for (ix, d) in DGTS.iter().enumerate() {
        let c = (central_coor.0 + d.0, central_coor.1 + d.1);

        match char_map.get(&c) {
            Some('#') => result |= 1 << ix,
            None if unknown == '#' => result |= 1 << ix,
            Some('.') => (),
            _ => (),
        }
    }

    result
}

fn inputs() -> anyhow::Result<(Vec<char>, HashMap<(i32, i32), char>)> {
    let input_string =
        std::fs::read_to_string("inputs/20_input.txt").context("Error while reading input")?;

    Ok(parse(&input_string))
}

fn parse(input_string: &str) -> (Vec<char>, HashMap<(i32, i32), char>) {
    let mut split = input_string.split("\n\n");

    let index = split.next().unwrap().chars().collect();

    let mut char_set = HashMap::new();
    for (y, line) in split.next().unwrap().lines().enumerate() {
        for (x, chr) in line.trim().chars().enumerate() {
            char_set.insert((y as i32, x as i32), chr);
        }
    }

    (index, char_set)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inputs() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###";

        let (index, char_map) = parse(input);

        let num = build_number((2, 2), &char_map, '.');
        assert_eq!(num, 34);

        let res = solve_01(char_map, &index, 1);

        assert_eq!(num_of_lighted(&res), 24);

        let res = solve_01(res, &index, 1);
        assert_eq!(num_of_lighted(&res), 35);
    }

    #[test]
    fn tst_build_number() {
        let map = [(1, 1)].into_iter().map(|e| (e, '#')).collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 1);

        let map = [(1, 1), (1, 0)].into_iter().map(|e| (e, '#')).collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 3);

        let map = [(1, 1), (1, 0), (1, -1)]
            .into_iter()
            .map(|e| (e, '#'))
            .collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 7);

        let map = [(1, 1), (1, 0), (1, -1), (0, 1)]
            .into_iter()
            .map(|e| (e, '#'))
            .collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 15);

        let map = [(1, 1), (1, 0), (1, -1), (0, 1), (0, 0)]
            .into_iter()
            .map(|e| (e, '#'))
            .collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 31);

        let map = [(1, 1), (1, 0), (1, -1), (0, 1), (0, 0), (0, -1)]
            .into_iter()
            .map(|e| (e, '#'))
            .collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 63);

        let map = [(1, 1), (1, 0), (1, -1), (0, 1), (0, 0), (0, -1), (-1, 1)]
            .into_iter()
            .map(|e| (e, '#'))
            .collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 127);

        let map = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, 0),
            (0, -1),
            (-1, 1),
            (-1, 0),
        ]
        .into_iter()
        .map(|e| (e, '#'))
        .collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 255);

        let map = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, 0),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ]
        .into_iter()
        .map(|e| (e, '#'))
        .collect();
        let num = build_number((0, 0), &map, '.');
        assert_eq!(num, 511);
    }
}
