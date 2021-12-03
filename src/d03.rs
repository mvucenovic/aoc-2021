use anyhow::Context;

fn inputs() -> anyhow::Result<Vec<BinaryCounter>> {
    let input_string =
        std::fs::read_to_string("inputs/03_input.txt").context("Error while reading input")?;

    Ok(input_string
        .split('\n')
        .map(|s| BinaryCounter::from_str(s))
        .collect::<Vec<_>>())
}

#[derive(Clone)]
struct BinaryCounter {
    digits_cnt: Vec<u16>,
    num_cnt: u16,
}

impl BinaryCounter {
    fn from_str(s: &str) -> Self {
        BinaryCounter {
            digits_cnt: s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect(),
            num_cnt: 0,
        }
    }

    fn add(&mut self, other: &BinaryCounter) {
        self.digits_cnt = self
            .digits_cnt
            .iter()
            .zip(other.digits_cnt.iter())
            .map(|(a, b)| a + b)
            .collect();
        self.num_cnt += 1;
    }

    fn to_number(&self) -> u32 {
        let str: String = self
            .digits_cnt
            .iter()
            .map(|&c| if c > self.num_cnt / 2 { '1' } else { '0' })
            .collect();
        u32::from_str_radix(&str, 2).unwrap()
    }
}

pub fn part_01() -> anyhow::Result<u32> {
    let nums = inputs()?;

    let counter = calc(&nums);

    let gamma = counter.to_number();
    let epsilon = !gamma & 0b111111111111;

    Ok(gamma * epsilon)
}

fn calc(nums: &[BinaryCounter]) -> BinaryCounter {
    let mut counter = BinaryCounter::from_str("000000000000");

    for num in nums.iter() {
        counter.add(num);
    }

    counter
}

pub fn part_02() -> anyhow::Result<u32> {
    let nums = inputs()?;

    let mut ix = 0;
    let mut n1 = nums.clone();
    while n1.len() > 1 {
        let counter = calc(&n1);
        let most_common = if counter.digits_cnt[ix] >= counter.num_cnt - counter.digits_cnt[ix] {
            1
        } else {
            0
        };
        n1 = n1
            .into_iter()
            .filter(|n| n.digits_cnt[ix] == most_common)
            .collect();
        ix += 1;
    }

    let mut ix = 0;
    let mut n2 = nums.clone();
    while n2.len() > 1 {
        let counter = calc(&n2);
        let least_common = if counter.digits_cnt[ix] >= counter.num_cnt - counter.digits_cnt[ix] {
            0
        } else {
            1
        };
        n2 = n2
            .into_iter()
            .filter(|n| n.digits_cnt[ix] == least_common)
            .collect();
        ix += 1;
    }

    let ox = n1.into_iter().next().unwrap().to_number();
    let co = n2.into_iter().next().unwrap().to_number();
    Ok(ox * co)
}
