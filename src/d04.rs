use anyhow::{anyhow, Context};

pub fn part_01() -> anyhow::Result<u32> {
    let (nums, mut cards) = inputs()?;

    for num in nums {
        for card in cards.iter_mut() {
            card.new_number(num);
            if card.is_bingo() {
                return Ok(num * card.unmarked_numbers_sum());
            }
        }
    }

    anyhow::bail!("No bingo found!")
}

pub fn part_02() -> anyhow::Result<u32> {
    let (nums, cards) = inputs()?;
    let mut cards = cards.into_iter().map(|c| (c, false)).collect::<Vec<_>>();

    let mut wins = 0;
    let cards_number = cards.len();

    for num in nums {
        for (card, won) in cards.iter_mut().filter(|(_, won)| !won) {
            card.new_number(num);
            if card.is_bingo() {
                wins += 1;
                *won = true;
                if wins == cards_number {
                    return Ok(num * card.unmarked_numbers_sum());
                }
            }
        }
    }
    anyhow::bail!("No bingo found!")
}

fn inputs() -> anyhow::Result<(Vec<u32>, Vec<BingoCard>)> {
    let input_string =
        std::fs::read_to_string("inputs/04_input.txt").context("Error while reading input")?;

    parse_inputs(&input_string)
}

fn parse_inputs(input: &str) -> anyhow::Result<(Vec<u32>, Vec<BingoCard>)> {
    let mut split = input.split('\n');

    let numbers_line = split
        .next()
        .ok_or(anyhow!("Missing line with input numbers"))?;
    let nums = numbers_line
        .split(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    let mut cards = Vec::new();
    while split.next().is_some() {
        let mut lines: [&str; 5] = [""; 5];
        for ix in 0..5 {
            lines[ix] = split
                .next()
                .ok_or(anyhow!("Bingo card is not formatted well"))?;
        }
        cards.push(BingoCard::parse_lines(lines)?);
    }

    Ok((nums, cards))
}

#[derive(Debug)]
struct BingoCard(Vec<Vec<(u32, bool)>>);

impl BingoCard {
    fn parse_lines(bingo: [&str; 5]) -> anyhow::Result<Self> {
        let mut rows = Vec::with_capacity(5);
        for line in bingo {
            let numbers = line
                .trim()
                .replace("  ", " ")
                .split(' ')
                .map(|n| n.trim().parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()?;
            rows.push(numbers.into_iter().map(|n| (n, false)).collect());
        }

        Ok(BingoCard(rows))
    }

    fn new_number(&mut self, num: u32) {
        for row in self.0.iter_mut() {
            let found = row.iter_mut().find(|&&mut (n, _)| n == num);
            if found.is_some() {
                found.unwrap().1 = true;
                return;
            }
        }
    }

    fn is_bingo(&self) -> bool {
        // check all rows
        let rows = &self.0;
        for row in rows.iter() {
            if row.iter().all(|n| n.1 == true) {
                return true;
            }
        }

        let dim = rows.len();
        for ix in 0..dim {
            let mut column_bingo = true;
            for iy in 0..dim {
                let (_, marked) = rows.get(iy).unwrap().get(ix).unwrap();
                if !marked {
                    column_bingo = false;
                    break;
                }
            }
            if column_bingo {
                return true;
            }
        }

        false
    }

    fn unmarked_numbers_sum(&self) -> u32 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|(_, marked)| *marked == false)
                    .map(|(n, _)| n)
                    .sum::<u32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_inputs() {
        let input_str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";

        let result = parse_inputs(input_str);

        assert_eq!(result.is_ok(), true);
        let (nums, cards) = result.unwrap();
        assert_eq!(nums.len(), 27);
        assert_eq!(cards.len(), 3);
        assert_eq!(cards.get(0).unwrap().0.len(), 5);
    }

    #[test]
    fn test_bingo_rows() {
        let inputs = [
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        let mut bingo = BingoCard::parse_lines(inputs).expect("bad parse");

        bingo.new_number(8);
        bingo.new_number(2);
        bingo.new_number(23);
        bingo.new_number(4);
        assert_eq!(bingo.is_bingo(), false);

        bingo.new_number(16);
        bingo.new_number(18);
        bingo.new_number(15);
        assert_eq!(bingo.is_bingo(), false);

        bingo.new_number(24);
        assert_eq!(bingo.is_bingo(), true);
    }

    #[test]
    fn test_bingo_collumns() {
        let inputs = [
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        let mut bingo = BingoCard::parse_lines(inputs).expect("bad parse");

        bingo.new_number(4);
        bingo.new_number(16);
        bingo.new_number(18);
        bingo.new_number(15);
        assert_eq!(bingo.is_bingo(), false);

        bingo.new_number(11);
        assert_eq!(bingo.is_bingo(), true);
    }
}
