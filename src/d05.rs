use std::cmp;
use std::collections::HashMap;

use anyhow::{anyhow, Context};

pub fn part_01() -> anyhow::Result<usize> {
    let lines = inputs()?;

    solve_01(lines)
}

fn solve_01(lines: Vec<Line>) -> anyhow::Result<usize> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        for dot in line.dots_on_line() {
            let cnt = map.entry(dot).or_insert(0);
            *cnt += 1;
        }
    }

    Ok(map.iter().filter(|(_, &v)| v >= 2).count())
}

pub fn part_02() -> anyhow::Result<usize> {
    let lines = inputs()?;

    solve_02(lines)
}

fn solve_02(lines: Vec<Line>) -> anyhow::Result<usize> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        for dot in line.dots_on_line_2() {
            let cnt = map.entry(dot).or_insert(0);
            *cnt += 1;
        }
    }

    Ok(map.iter().filter(|(_, &v)| v >= 2).count())
}

fn inputs() -> anyhow::Result<Vec<Line>> {
    let input_string =
        std::fs::read_to_string("inputs/05_input.txt").context("Error while reading input")?;

    parse_inputs(&input_string)
}

fn parse_inputs(input_str: &str) -> anyhow::Result<Vec<Line>> {
    input_str
        .split('\n')
        .map(|s| Line::try_from_str(s))
        .collect()
}

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn try_from_str(line: &str) -> anyhow::Result<Self> {
        let mut s1 = line.trim().split(" -> ");

        let start_str = s1
            .next()
            .ok_or(anyhow!("Line {} is not in the valid format", line))?;
        let end_str = s1
            .next()
            .ok_or(anyhow!("Line {} is not in the valid format", line))?;

        let n = format!("{},{}", start_str.trim(), end_str.trim())
            .split(',')
            .map(|u| u.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        anyhow::ensure!(n.len() == 4, "Line not valid");
        Ok(Line {
            start: (*n.get(0).unwrap(), *n.get(1).unwrap()),
            end: (*n.get(2).unwrap(), *n.get(3).unwrap()),
        })
    }

    fn dots_on_line(&self) -> Vec<(i32, i32)> {
        if self.start.0 == self.end.0 {
            // vertical line
            let y1 = cmp::min(self.start.1, self.end.1);
            let y2 = cmp::max(self.start.1, self.end.1);

            (y1..=y2).map(|y| (self.start.0, y)).collect()
        } else if self.start.1 == self.end.1 {
            // horizontal line
            let x1 = cmp::min(self.start.0, self.end.0);
            let x2 = cmp::max(self.start.0, self.end.0);

            (x1..=x2).map(|x| (x, self.start.1)).collect()
        } else {
            // non-vertical / non-horizontal
            vec![]
        }
    }

    fn dots_on_line_2(&self) -> Vec<(i32, i32)> {
        if (self.start.0 - self.end.0).abs() == (self.start.1 - self.end.1).abs() {
            // diagonal (45 degree angle)
            let dx = (self.end.0 - self.start.0).signum();
            let dy = (self.end.1 - self.start.1).signum();

            let mut x = self.start.0;
            let mut y = self.start.1;

            let mut res = vec![(x, y)];
            while (x, y) != (self.end.0, self.end.1) {
                x += dx;
                y += dy;
                res.push((x, y));
            }
            res
        } else {
            self.dots_on_line()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_inputs() {
        let res = Line::try_from_str("18,0 -> 0,8");

        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res.start, (18, 0));
        assert_eq!(res.end, (0, 8));
    }

    #[test]
    fn test_inputs_p01() {
        let lines = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        let lines = parse_inputs(lines).expect("Parsing failed");
        let res = solve_01(lines);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 5);
    }

    #[test]
    fn test_inputs_p02() {
        let lines = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

        let lines = parse_inputs(lines).expect("Parsing failed");
        let res = solve_02(lines);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 12);
    }
}
