use std::fmt::Debug;

// this is my input
const PLAYER_1: u32 = 6;
const PLAYER_2: u32 = 9;

const BOARD_SIZE: u32 = 10;

pub fn part_01() -> anyhow::Result<u32> {
    let players = [Player::new(PLAYER_1), Player::new(PLAYER_2)];

    let (players, winner, dice) = solve_01(players);

    let looser = (winner + 1) % 2;

    Ok(players[looser].score * dice.number_of_rolls)
}

pub fn part_02() -> anyhow::Result<u64> {
    let players = [Player::new(PLAYER_1), Player::new(PLAYER_2)];

    let res = solve_02(players);

    Ok(std::cmp::max(res[0], res[1]))
}

fn solve_01(mut players: [Player; 2]) -> ([Player; 2], usize, DeterministicDice) {
    let mut dice = DeterministicDice::new();
    let winner;
    'game: loop {
        for ix in 0..=1 {
            let player_roll = dice.roll() + dice.roll() + dice.roll();
            players[ix].turn_move(player_roll);
            if players[ix].score >= 1000 {
                winner = ix;
                break 'game;
            }
        }
    }
    (players, winner, dice)
}

/*
    1 1 1 = 3 (1)

    2 1 1 = 4 (3)

    2 2 1 = 5 (3)
    3 1 1 = 5 (3)

    2 2 2 = 6 (1)
    3 2 1 = 6 (6)

    3 2 2 = 7 (3)
    3 3 1 = 7 (3)

    3 3 2 = 8 (3)

    3 3 3 (1)
*/

const DICE_ROLL_CHANCES: [(u32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn solve_02(players: [Player; 2]) -> [u64; 2] {
    fn recurse(ps: [Player; 2], turn: usize) -> [u64; 2] {
        let active_player = turn % 2;
        let mut wins = [0, 0];

        for (dice_roll, times_happening) in DICE_ROLL_CHANCES {
            let mut ps2 = ps.clone();
            ps2[active_player].turn_move(dice_roll);

            if ps2[active_player].score >= 21 {
                // this game ends (and happens `times_happening` times)
                wins[active_player] += times_happening;
            } else {
                // game goes on with recursion
                let res = recurse(ps2, turn + 1);
                wins[0] += times_happening * res[0];
                wins[1] += times_happening * res[1];
            }
        }
        wins
    }

    recurse(players, 0)
}

#[derive(Debug)]
struct DeterministicDice {
    number_of_rolls: u32,
}

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice { number_of_rolls: 0 }
    }

    fn roll(&mut self) -> u32 {
        self.number_of_rolls += 1;
        let mut ret = self.number_of_rolls;
        if ret > 100 {
            ret = ret % 100;
        }
        ret
    }
}

#[derive(Debug, Clone)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(starting_pos: u32) -> Self {
        Player {
            pos: starting_pos,
            score: 0,
        }
    }

    fn turn_move(&mut self, moves: u32) {
        let mut pos = self.pos + moves % BOARD_SIZE;
        if pos > BOARD_SIZE {
            pos = pos % BOARD_SIZE;
        }

        self.score += pos;
        self.pos = pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inputs() {
        let players = [Player::new(4), Player::new(8)];
        let (players, winner, dice) = solve_01(players);

        assert_eq!(winner, 0);
        assert_eq!(players[1].score * dice.number_of_rolls, 739785);
    }

    #[test]
    fn test_inputs2() {
        let players = [Player::new(4), Player::new(8)];
        let res = solve_02(players);

        assert_eq!(res, [444356092776315, 341960390180808]);
    }
}
