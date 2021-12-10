use anyhow::Context;

use std::collections::VecDeque;

fn inputs() -> anyhow::Result<Vec<String>> {
    let input_string =
        std::fs::read_to_string("inputs/10_input.txt").context("Error while reading input")?;

    Ok(input_string
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>())
}

pub fn part_01() -> anyhow::Result<u32> {
    let inputs = inputs()?;

    let mut res = 0;
    for input in inputs {
        match find_corrupted_char(&input) {
            Some(')') => res += 3,
            Some(']') => res += 57,
            Some('}') => res += 1197,
            Some('>') => res += 25137,
            _ => (),
        }
    }

    Ok(res)
}

fn find_corrupted_char(line: &str) -> Option<char> {
    let mut deque = VecDeque::new();
    for ch in line.chars() {
        if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
            deque.push_front(ch);
        } else {
            if let Some(open) = deque.pop_front() {
                if !are_pairs(open, ch) {
                    return Some(ch);
                }
            }
        }
    }
    None
}

fn are_pairs(left: char, right: char) -> bool {
    match (left, right) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn get_opens(line: &str) -> VecDeque<char> {
    let mut deque = VecDeque::new();
    for ch in line.chars() {
        if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
            deque.push_front(ch);
        } else {
            deque.pop_front();
        }
    }
    deque
}

fn calc_score(deque: &mut VecDeque<char>) -> u64 {
    let mut score = 0;
    while !deque.is_empty() {
        score *= 5;
        score += match deque.pop_front() {
            Some('(') => 1,
            Some('[') => 2,
            Some('{') => 3,
            Some('<') => 4,
            _ => panic!("at the disco"),
        };
    }
    score
}

pub fn part_02() -> anyhow::Result<u64> {
    let inputs = inputs()?;

    let incompletes: Vec<String> = inputs
        .into_iter()
        .filter(|s| find_corrupted_char(s).is_none())
        .collect();

    let mut sums: Vec<u64> = incompletes
        .iter()
        .map(|l| calc_score(&mut get_opens(l)))
        .collect();

    sums.sort();

    Ok(sums[sums.len() / 2])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_corrupted_char() {
        assert_eq!(find_corrupted_char("{([(<{}[<>[]}>{[]{[(<()>"), Some('}'));
        assert_eq!(find_corrupted_char("[[<[([]))<([[{}[[()]]]"), Some(')'));
        assert_eq!(find_corrupted_char("[{[{({}]{}}([{[{{{}}([]"), Some(']'));
        assert_eq!(find_corrupted_char("[<(<(<(<{}))><([]([]()"), Some(')'));
        assert_eq!(find_corrupted_char("<{([([[(<>()){}]>(<<{{"), Some('>'));
    }
}
