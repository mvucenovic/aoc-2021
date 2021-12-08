use anyhow::Context;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_01() -> anyhow::Result<usize> {
    let inputs = inputs()?;

    Ok(inputs
        .iter()
        .map(|input| {
            input
                .1
                .iter()
                .filter(|d| d.len() == 2 || d.len() == 4 || d.len() == 3 || d.len() == 7)
                .count()
        })
        .sum())
}

/*
 0 - 6seg
 9 - 6seg
 6 - 6seg

 2 - 5seg
 3 - 5seg
 5 - 5seg

 1 - 2seg

 7 - 3seg

 4 - 4seg

 8 - 7seg


we know 1 4, 7 and 8

7 - 1 -> letter on top (a)
4 - 1 -> b + d

-----

find a 6 seg that contains 7 + 4 => that's 9, so we get (g)

find a 6 seg that doesn't contain a 1 => that's 6, and now we get (e)

last 6 seg is a 0 (we can know d after that)

-----------

3 = 7 + d + g
5 = 6 - e
2 is the last one \o/

*/

pub fn part_02() -> anyhow::Result<usize> {
    let inputs = inputs()?;

    let mut digits: [HashSet<char>; 10] = Default::default();
    let mut segments: [HashSet<char>; 7] = Default::default();

    let mut sum = 0;

    for input in inputs {
        let (mut signal_patterns, mut result_digits) = input;

        digits[1] = conv_to_set(
            signal_patterns.remove(
                signal_patterns
                    .iter()
                    .find_position(|s| s.len() == 2)
                    .unwrap()
                    .0,
            ),
        );
        digits[4] = conv_to_set(
            signal_patterns.remove(
                signal_patterns
                    .iter()
                    .find_position(|s| s.len() == 4)
                    .unwrap()
                    .0,
            ),
        );
        digits[7] = conv_to_set(
            signal_patterns.remove(
                signal_patterns
                    .iter()
                    .find_position(|s| s.len() == 3)
                    .unwrap()
                    .0,
            ),
        );
        digits[8] = conv_to_set(
            signal_patterns.remove(
                signal_patterns
                    .iter()
                    .find_position(|s| s.len() == 7)
                    .unwrap()
                    .0,
            ),
        );

        // 7 - 1 -> segment on top (a)
        segments[0] = &digits[7] - &digits[1];

        // find a 6 seg that contains 7 + 4 => that's 9, so we get (g)
        digits[9] = conv_to_set(
            signal_patterns.remove(
                signal_patterns
                    .iter()
                    .find_position(|&s| {
                        s.len() == 6
                            && conv_to_set(s.clone()).is_superset(&digits[7])
                            && conv_to_set(s.clone()).is_superset(&digits[4])
                    })
                    .unwrap()
                    .0,
            ),
        );

        // find a 6 seg that doesn't contain a 1 => that's 6, and now we get (e)
        digits[6] = conv_to_set(
            signal_patterns.remove(
                signal_patterns
                    .iter()
                    .find_position(|&s| {
                        s.len() == 6 && !conv_to_set(s.clone()).is_superset(&digits[1])
                    })
                    .unwrap()
                    .0,
            ),
        );

        // last 6 seg is a 0 (we can know d after that)
        digits[0] = conv_to_set(
            signal_patterns.remove(
                signal_patterns
                    .iter()
                    .find_position(|&s| s.len() == 6)
                    .unwrap()
                    .0,
            ),
        );

        // a - 0, b - 1, c - 2, d -3, e-4, f-5, g-6

        // segment e = 8 - 9
        segments[4] = &digits[8] - &digits[9];

        // segment d = 8 - 0
        segments[3] = &digits[8] - &digits[0];

        // segment g = 9 - 4 - 'a'
        segments[6] = &(&digits[9] - &digits[4]) - &segments[0];

        // 3 = 7 + d + g
        digits[3] = digits[7]
            .union(&segments[3].union(&segments[6]).cloned().collect())
            .cloned()
            .collect();

        // 5 = 6 - e
        digits[5] = &digits[6] - &segments[4];

        // 2 is the last 5 segment one
        digits[2] = signal_patterns
            .iter()
            .filter(|&s| s.len() == 5)
            .map(|s| conv_to_set(s.to_owned()))
            .find(|dig| *dig != digits[3] && *dig != digits[5])
            .unwrap();

        result_digits.reverse();
        let mut multiplier = 1;
        let mut res = 0;
        for result_digit in result_digits {
            let d = conv_to_set(result_digit.clone());

            let digit_value = digits
                .iter()
                .find_position(|&digit| *digit == d)
                .expect(&format!("panic {}", result_digit))
                .0;

            res += digit_value * multiplier;
            multiplier *= 10;
        }
        sum += res;
    }

    Ok(sum)
}

fn conv_to_set(s: String) -> HashSet<char> {
    s.chars().collect::<HashSet<_>>()
}

fn inputs() -> anyhow::Result<Vec<(Vec<String>, Vec<String>)>> {
    let input_string =
        std::fs::read_to_string("inputs/08_input.txt").context("Error while reading input")?;

    let mut res = vec![];
    for line in input_string.lines() {
        let mut split = line.split(" | ");

        let signal_patterns = split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        let digits = split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        res.push((signal_patterns, digits))
    }

    Ok(res)
}
