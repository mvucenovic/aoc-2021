use anyhow::Context;

pub fn part_01() -> anyhow::Result<usize> {
    let starting = inputs()?;

    Ok(solve(80, &starting))
}

pub fn part_02() -> anyhow::Result<usize> {
    let starting = inputs()?;

    Ok(solve(256, &starting))
}

fn inputs() -> anyhow::Result<Vec<usize>> {
    let input_string =
        std::fs::read_to_string("inputs/06_input.txt").context("Error while reading input")?;

    Ok(input_string.split(',').map(|n| n.parse()).collect::<Result<_,_>>()?)
}

fn solve(max_days: usize, inputs: &[usize]) -> usize {
    let mut states_cnt = [0usize; 9];

    for state in inputs {
        states_cnt[*state] += 1;
    }

    for day in 0..max_days {
        iteration(&mut states_cnt);
    }

    states_cnt.iter().sum()
}

fn iteration(states_cnt: &mut [usize; 9]) {
    let zeros = states_cnt[0];
    for i in 0..8 {
        states_cnt[i] = states_cnt[i + 1];
    }
    states_cnt[6] += zeros;
    states_cnt[8] = zeros;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cast_p01() {
        let inputs = vec![3,4,3,1,2];
        
        let res = solve(18, &inputs);
        assert_eq!(res, 26);

        let res = solve(80, &inputs);
        assert_eq!(res, 5934);

        let res = solve(256, &inputs);
        assert_eq!(res, 26984457539);
    }
}
