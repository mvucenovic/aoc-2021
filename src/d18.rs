use std::fmt::Display;

use itertools::Itertools;

use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Clone, Debug)]
enum Snailfish {
    Number(u32),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    fn pair_of(a: u32, b: u32) -> Self {
        Snailfish::Pair(
            Box::new(Snailfish::Number(a)),
            Box::new(Snailfish::Number(b)),
        )
    }

    fn from_str(line: &str) -> Self {
        let (_, fish) = parse_snailfish(line).expect("Bad Input");
        fish
    }

    fn magnitude(&self) -> u64 {
        match self {
            Snailfish::Number(val) => *val as u64,
            Snailfish::Pair(s1, s2) => 3 * s1.magnitude() + 2 * s2.magnitude(),
        }
    }

    fn reduce(&mut self) {
        loop {
            let exploded = self.explode();
            if exploded {
                continue;
            }

            let splited = self.split();
            if splited {
                continue;
            }

            break;
        }
    }

    fn find_and_explode_index(&mut self, index: usize, depth: usize) -> (bool, usize, u32, u32) {
        match self {
            Snailfish::Pair(s1, s2) if depth == 4 => match (*s1.clone(), *s2.clone()) {
                (Snailfish::Number(v1), Snailfish::Number(v2)) => {
                    *self = Snailfish::Number(0);
                    return (true, index, v1, v2);
                }
                (Snailfish::Number(_), Snailfish::Pair(mut s1, mut s2)) => {
                    let new_index = index + 1; // left one is plain number, and should be indexed
                    let (_, new_ix, _, _) = s1.find_and_explode_index(new_index, depth + 1);
                    let (_, new_ix, _, _) = s2.find_and_explode_index(new_ix, depth + 1);
                    return (false, new_ix, 0, 0);
                }
                (Snailfish::Pair(mut s1, mut s2), Snailfish::Number(_)) => {
                    let (_, new_ix, _, _) = s1.find_and_explode_index(index, depth + 1);
                    let (_, new_ix, _, _) = s2.find_and_explode_index(new_ix, depth + 1);
                    let new_ix = new_ix + 1; // right one is plain number, and should be indexed
                    return (false, new_ix + 1, 0, 0);
                }
                (Snailfish::Pair(mut s1, mut s2), Snailfish::Pair(mut s3, mut s4)) => {
                    let (_, new_ix, _, _) = s1.find_and_explode_index(index, depth + 1);
                    let (_, new_ix, _, _) = s2.find_and_explode_index(new_ix, depth + 1);
                    let (_, new_ix, _, _) = s3.find_and_explode_index(new_ix, depth + 1);
                    let (_, new_ix, _, _) = s4.find_and_explode_index(new_ix, depth + 1);
                    return (false, new_ix, 0, 0);
                }
            },
            Snailfish::Pair(s1, s2) => {
                let (found, new_ix, left, right) = s1.find_and_explode_index(index, depth + 1);
                if found {
                    return (found, new_ix, left, right);
                }

                s2.find_and_explode_index(new_ix, depth + 1)
            }
            Snailfish::Number(_) => return (false, index + 1, 0, 0),
        }
    }

    fn explode_around_index(
        &mut self,
        cur_ix: usize,
        explosion_ix: usize,
        left: u32,
        right: u32,
    ) -> (bool, usize) {
        match self {
            Snailfish::Number(v) if cur_ix + 1 == explosion_ix => {
                *v += left;
                return (false, cur_ix + 1);
            }
            Snailfish::Number(v) if cur_ix == explosion_ix + 1 => {
                *v += right;
                return (true, cur_ix + 1);
            }
            Snailfish::Number(_) => return (false, cur_ix + 1),
            Snailfish::Pair(s1, s2) => {
                let (finished, new_ix) = s1.explode_around_index(cur_ix, explosion_ix, left, right);
                if finished {
                    return (finished, new_ix);
                }
                return s2.explode_around_index(new_ix, explosion_ix, left, right);
            }
        }
    }

    fn explode(&mut self) -> bool {
        let (found, ix, l, r) = self.find_and_explode_index(0, 0);
        if !found {
            return false;
        }
        self.explode_around_index(0, ix, l, r);
        return true;
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Number(v) if *v >= 10 => {
                let left = *v / 2;
                let right = *v / 2 + *v % 2;
                *self = Snailfish::pair_of(left, right);
                true
            }
            Snailfish::Number(_) => false,
            Snailfish::Pair(s1, s2) => {
                if s1.split() == true {
                    true
                } else {
                    s2.split()
                }
            }
        }
    }

    fn add(self, other: Snailfish) -> Snailfish {
        Snailfish::Pair(Box::new(self), Box::new(other))
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Snailfish::Number(v) => write!(f, "{}", *v),
            Snailfish::Pair(s1, s2) => write!(f, "[{},{}]", s1, s2),
        }
    }
}

fn parse_snailfish(line: &str) -> IResult<&str, Snailfish> {
    let parse_number_fish = map(complete::u32, |num| Snailfish::Number(num));

    alt((parse_number_fish, parse_pair_fish))(line)
}

fn parse_pair_fish(line: &str) -> IResult<&str, Snailfish> {
    let parse_pair = separated_pair(parse_snailfish, tag(","), parse_snailfish);

    let map_and_parse_pair = map(parse_pair, |(s1, s2)| {
        Snailfish::Pair(Box::new(s1), Box::new(s2))
    });

    delimited(tag("["), map_and_parse_pair, tag("]"))(line)
}

pub fn part_01() -> anyhow::Result<u64> {
    let fishes = inputs()?;

    Ok(solve_01(fishes).magnitude())
}

fn solve_01(mut fishes: Vec<Snailfish>) -> Snailfish {
    let mut res = fishes.remove(0);
    res.reduce();

    for other in fishes {
        res = res.add(other);
        res.reduce();
    }

    res
}

pub fn part_02() -> anyhow::Result<u64> {
    let fishes = inputs()?;

    let max = solve_02(fishes);

    Ok(max) // 14429 too high
}

fn solve_02(fishes: Vec<Snailfish>) -> u64 {
    fishes
        .into_iter()
        .permutations(2)
        .map(|mut perm| {
            let mut added_fishes = perm.remove(1).add(perm.remove(0));
            added_fishes.reduce();
            added_fishes.magnitude()
        })
        .max()
        .unwrap()
}

fn inputs() -> anyhow::Result<Vec<Snailfish>> {
    let input_string =
        std::fs::read_to_string("inputs/18_input.txt").context("Error while reading input")?;

    input_string.lines().map(read_line).collect()
}

fn read_line(line: &str) -> anyhow::Result<Snailfish> {
    Ok(Snailfish::from_str(line))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser_and_magnitude() {
        let inputs = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (fish_str, result) in inputs {
            let fish = Snailfish::from_str(fish_str);
            assert_eq!(fish.magnitude(), result);
        }
    }

    #[test]
    fn test_explosions() {
        let inputs = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (before_explosion, after_explosion) in inputs {
            let mut s = Snailfish::from_str(before_explosion);
            let exploded = s.explode();
            assert_eq!(exploded, true);
            let s_displayed = format!("{}", s);
            assert_eq!(&s_displayed, after_explosion);
        }
    }

    #[test]
    fn test_splits() {
        let inputs = [
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];

        for (before, after) in inputs {
            let mut s = Snailfish::from_str(before);
            let split = s.split();
            assert_eq!(split, true);
            let s_displayed = format!("{}", s);
            assert_eq!(&s_displayed, after);
        }
    }

    #[test]
    fn test_part_01() {
        let inputs = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];

        let inputs = inputs
            .into_iter()
            .map(Snailfish::from_str)
            .collect::<Vec<_>>();

        let res = solve_01(inputs);

        assert_eq!(res.magnitude(), 4140);
    }

    #[test]
    fn test_part_02() {
        let inputs = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];

        let inputs = inputs
            .into_iter()
            .map(Snailfish::from_str)
            .collect::<Vec<_>>();

        let res = solve_02(inputs);

        assert_eq!(res, 3993);
    }
}
