use anyhow::Context;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part_01() -> anyhow::Result<u32> {
    let starting = inputs()?;

    Ok(solve_01(&starting)[starting.len() - 1][starting[0].len() - 1])
}

pub fn part_02() -> anyhow::Result<u32> {
    let starting = inputs()?;

    let quintupled = quintuple(starting);

    let h = quintupled.len() as isize;
    let w = quintupled[0].len() as isize;

    let res = a_star2(&quintupled, (0, 0), ((h - 1) as i16, (w - 1) as i16)).unwrap();
    Ok(res) // 2882 too high
}

fn a_start(risc_map: &Vec<Vec<u32>>, start: (isize, isize), goal: (isize, isize)) -> Option<u32> {
    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, heuristic(start, goal));

    let mut open_set = HashSet::new();
    open_set.insert((heuristic(start, goal), start));

    while let Some(&curr) = open_set.iter().next() {
        open_set.remove(&curr);

        let (f_score, pos) = curr;
        if pos == goal {
            return Some(f_score);
        }
        let (y, x) = pos;
        let g_score = g_scores[&pos];

        for (dy, dx) in itertools::iproduct!(-1..=1, -1..=1) {
            let (y, x) = (y + dy, x + dx);
            let risc =
                if let Some(&risc) = risc_map.get(y as usize).and_then(|row| row.get(x as usize)) {
                    risc
                } else {
                    continue;
                };
            let new_pos = (y, x);
            let tentative_g_score = g_score + risc;
            let old_g_score = g_scores.get(&new_pos).copied();
            if tentative_g_score < old_g_score.unwrap_or(u32::MAX) {
                if old_g_score.is_some() {
                    open_set.remove(&(f_scores[&new_pos], new_pos));
                }
                let new_f_score = tentative_g_score + heuristic(new_pos, goal);
                f_scores.insert(new_pos, new_f_score);
                g_scores.insert(new_pos, tentative_g_score);
                open_set.insert((new_f_score, new_pos));
            }
        }
    }

    None
}

fn heuristic(start: (isize, isize), goal: (isize, isize)) -> u32 {
    (start.1 - goal.1).abs() as u32 + (start.0 - goal.0).abs() as u32
}

fn solve_01(risc_map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result_map = risc_map.clone();
    result_map[0][0] = 0;

    for x in 1..risc_map[0].len() {
        result_map[0][x] = result_map[0][x - 1] + risc_map[0][x];
    }

    for y in 1..risc_map.len() {
        result_map[y][0] = result_map[y - 1][0] + risc_map[y][0];
    }

    for y in 1..risc_map.len() {
        for x in 1..risc_map.len() {
            result_map[y][x] = std::cmp::min(
                result_map[y][x - 1] + risc_map[y][x],
                result_map[y - 1][x] + risc_map[y][x],
            );
        }
    }
    result_map
}

fn inputs() -> anyhow::Result<Vec<Vec<u32>>> {
    let input_string =
        std::fs::read_to_string("inputs/15_input.txt").context("Error while reading input")?;

    parse(&input_string)
}

fn parse(input_str: &str) -> anyhow::Result<Vec<Vec<u32>>> {
    input_str
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).context("not a valid digit"))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}

fn quintuple(input_map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let max_y = input_map.len();
    let max_x = input_map[0].len();

    let mut res = vec![vec![0; max_x * 5]; max_y * 5];

    for y in 0..max_y {
        for x in 0..max_x {
            for offset_y in 0..5 {
                for offset_x in 0..5 {
                    let cur_y = y + offset_y * max_y;
                    let cur_x = x + offset_x * max_x;
                    res[cur_y][cur_x] = input_map[y][x] + offset_y as u32 + offset_x as u32;
                    if res[cur_y][cur_x] >= 10 {
                        res[cur_y][cur_x] = res[cur_y][cur_x] % 10 + 1;
                    }
                }
            }
        }
    }

    res
}

fn pprint(map: &Vec<Vec<u32>>) {
    for y in 0..map.len() {
        println!("{}", map[y].iter().join(""))
    }
}

fn h(pos: (i16, i16), goal: (i16, i16)) -> u32 {
    (pos.0 - goal.0).abs() as u32 + (pos.1 - goal.1).abs() as u32
}

fn a_star2(grid: &Vec<Vec<u32>>, start: (i16, i16), goal: (i16, i16)) -> Option<u32> {
    let start_f_score = h(start, goal);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, start_f_score);

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut open_set = std::collections::BTreeSet::new();
    open_set.insert((start_f_score, start));

    while let Some(&curr) = open_set.iter().next() {
        open_set.remove(&curr);

        let (f_score, pos) = curr;
        if pos == goal {
            return Some(f_score);
        }
        let (y, x) = pos;
        let g_score = g_scores[&pos];

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (y, x) = (y + dy, x + dx);
            let danger =
                if let Some(&danger) = grid.get(y as usize).and_then(|row| row.get(x as usize)) {
                    danger
                } else {
                    continue;
                };
            let new_pos = (y, x);
            let tentative_g_score = g_score + danger;
            let old_g_score = g_scores.get(&new_pos).copied();
            if tentative_g_score < old_g_score.unwrap_or(u32::MAX) {
                if old_g_score.is_some() {
                    open_set.remove(&(f_scores[&new_pos], new_pos));
                }

                let new_f_score = tentative_g_score + h(new_pos, goal);
                f_scores.insert(new_pos, new_f_score);
                g_scores.insert(new_pos, tentative_g_score);
                open_set.insert((new_f_score, new_pos));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input_p1() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";

        let risc_map = parse(input).unwrap();

        let results_map = solve_01(&risc_map);

        assert_eq!(results_map[risc_map.len() - 1][risc_map[0].len() - 1], 40)
    }

    #[test]
    fn test_input_p2() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";

        let risc_map = parse(input).unwrap();
        let risc_map = quintuple(risc_map);

        let results_map = solve_01(&risc_map);
        assert_eq!(results_map[risc_map.len() - 1][risc_map[0].len() - 1], 315)
    }
}
