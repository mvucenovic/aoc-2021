use anyhow::Context;
use std::{collections::HashMap, panic};

pub fn part_01() -> anyhow::Result<usize> {
    let inputs = inputs()?;

    let mut state = to_map(inputs);

        let res = solve_01(&mut state, 2);

    Ok(res)

}

pub fn part_02() -> anyhow::Result<usize> {
    let inputs = inputs()?;

    let mut state = to_map(inputs);

    let a = state.remove(&(2, 2)).unwrap();
    state.insert((2, 2),'D');
    state.insert((2, 3),'D');
    state.insert((2, 4),a);

    let a = state.remove(&(4, 2)).unwrap();
    state.insert((4, 2),'C');
    state.insert((4, 3),'B');
    state.insert((4, 4),a);

    let a = state.remove(&(6, 2)).unwrap();
    state.insert((6, 2),'B');
    state.insert((6, 3),'A');
    state.insert((6, 4),a);

    let a = state.remove(&(8, 2)).unwrap();
    state.insert((8, 2),'A');
    state.insert((8, 3),'C');
    state.insert((8, 4),a);

        let res = solve_01(&mut state, 4);

    Ok(res)
}


fn inputs() -> anyhow::Result<Vec<Amphypod>> {
    let input_string =
        std::fs::read_to_string("inputs/23_input.txt").context("Error while reading input")?;

    parse(&input_string)
}

fn amphypod_finished(a: &Amphypod) -> bool {
    match a {
        ((2, y), 'A') if *y > 0 => true,
        ((4, y), 'B') if *y > 0 => true,
        ((6, y), 'C') if *y > 0 => true,
        ((8, y), 'D') if *y > 0 => true,
        _ => false
    }
}

fn amphypod_finished_actual(a: &Amphypod, state: &HashMap<(usize, usize), char>, max_depth: usize) -> bool {
    let x = match a {
        ((2, y), 'A') if *y > 0 => 2,
        ((4, y), 'B') if *y > 0 => 4,
        ((6, y), 'C') if *y > 0 => 6,
        ((8, y), 'D') if *y > 0 => 8,
        _ => return false
    };

    for y in 1..=max_depth {
        let a2 = state.get(&(x, y));
        if let Some(a2) = a2 {
            if *a2 != a.1 {
                return false;
            }
        }
    }

    return true;
}

fn all_finished(state: &HashMap<(usize, usize), char>) -> bool {
    for (k, v) in state {
        if !amphypod_finished(&(*k, *v)) {
            return false;
        }
    }
    true
}

fn solve_01(state: &mut HashMap<(usize, usize), char>, max_depth: usize) -> usize {

    fn recurse(state: &mut HashMap<(usize, usize), char>, cur_cost: usize, best_so_far: usize, cache: &mut HashMap<Vec<Amphypod>, usize>, max_depth: usize) -> usize {

        if all_finished(state) {
            // println!("bsof = {}, cur ={}, stt{:?}", best_so_far, cur_cost, state);
            return cur_cost;
        }

        if cur_cost > best_so_far {
            return best_so_far;
        }

        let mut moves = vec![];

        for (k, v) in state.iter() {
            let a = (*k, *v);
            if amphypod_finished_actual(&a, state, max_depth) {
                continue;
            }
            
            possible_moves(state, *k, max_depth)
                .into_iter()
                .map(|a2| (a, a2, distance_cost(&a, &a2)))
                .for_each(|el| moves.push(el));
        }

        let mut new_best_so_far = best_so_far;

        // sort by distance
        moves.sort_by(|m1, m2| m1.2.cmp(&m2.2));

        for (a1, a2, distance) in moves {

            state.remove(&a1.0);
            state.insert(a2.0, a2.1);

            let next_cost = cur_cost + distance;
            let cache_line  = to_vec(state);
            if let Some(prev_cost) = cache.get(&cache_line) {
                if *prev_cost <= next_cost {
                    state.remove(&a2.0);
                    state.insert(a1.0, a1.1);
                    continue;
                }
            }
            cache.insert(cache_line, next_cost);

            let res = recurse(state, next_cost, new_best_so_far, cache, max_depth);
            new_best_so_far = min(res, new_best_so_far);

            state.remove(&a2.0);
            state.insert(a1.0, a1.1);
        }

        new_best_so_far
    }

    let mut cache = HashMap::new();
    recurse(state, 0, usize::MAX, &mut cache, max_depth)
}

fn possible_moves(state: &HashMap<(usize, usize), char>, ix: (usize, usize), max_depth: usize) -> Vec<Amphypod> {

    let a = *state.get(&ix).unwrap();

    let mut moves = vec![];

    if amphypod_finished_actual(&(ix, a), state, max_depth) {
        return moves;
    }

    if ix.1 > 0 {
        // it is in the side room so we go to proper side room OR corridor 

        let mut ix2 = ix;
        // fist we go up
        while ix2.1 > 0 {
            ix2.1 -= 1;
            
            if state.contains_key(&ix2) {
                // collision, cannot go up
                return moves;
            }
        }

        // we are in front of side room, we can go left and right

        // can we go to proper side room
        let to_the_sideroom = go_to_sideroom(ix2, a, state, max_depth);

        if let Some(to_the_sideroom) = to_the_sideroom {
            return vec![to_the_sideroom];
        }
        // we cannot go to the sideroom directly

        let mut ix_left = ix2;
        while ix_left.0 > 0 {
            ix_left.0 -= 1;

            if state.contains_key(&ix_left) {
                // collision, cannot go left any more
                break;
            }

            if ix_left.0 != 2 && ix_left.0 != 4 && ix_left.0 != 6 && ix_left.0 != 8 {
                moves.push((ix_left, a));
            }
        }

        let mut ix_right = ix2;
        while ix_right.0 < 10 {
            ix_right.0 += 1;

            if state.contains_key(&ix_right) {
                // collision, cannot go left any more
                break;
            }

            if ix_right.0 != 2 && ix_right.0 != 4 && ix_right.0 != 6 && ix_right.0 != 8 {
                moves.push((ix_right, a));
            }
        }

    } else {
        // it is in the corridor, so we go to correct side room

        let direct = go_to_sideroom(ix, a, state, max_depth);
        if let Some(a) = direct {
            moves.push(a);
        }
    }

    moves
}

fn go_to_sideroom(ix: (usize, usize), a:char, state: &HashMap<(usize, usize), char>, max_depth: usize) -> Option<Amphypod> {
    let final_x = sideroom_x(a);
        for y in 1..=max_depth {
            let a2 = state.get(&(final_x, y));
            if a2.is_some() && *a2.unwrap() != a {
                return None;
            }
        }

        // see if there is anyone on the corridor blocking us
        for x in min(ix.0, final_x)..=max(ix.0, final_x) {
            if state.contains_key(&(x, 0)) && (x, 0) != ix {
                return None;
            }
        }

        // there is not, so we can move downwards
        for y in (1..=max_depth).rev() {
            if state.contains_key(&(final_x, y)) {
                continue;
            }
            return Some(((final_x, y), a));
        }
        None
}

use std::cmp::{min, max};

fn sideroom_x(a: char) -> usize {
    match a {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => panic!("wrong letter")
    }
}

fn distance_cost(a1: &Amphypod, a2: &Amphypod) -> usize {

    let x = (a1.0.0 as isize - a2.0.0 as isize).abs() as usize;
    let y = a1.0.1 + a2.0.1;

    let cost = match a1.1 {
        'A' => 1,
        'B' => 10, 
        'C' => 100,
        'D' => 1000,
        _ => panic!("unexpected letter")
    };

    (x + y) * cost
}


type Amphypod = ((usize, usize), char);

fn parse(s: &str) -> anyhow::Result<Vec<Amphypod>> {
    let mut lines = s.lines().skip(1);
    let corridor_line = lines.next().context("bad input")?.trim();
    let first_line = lines.next().context("bad input")?.trim();
    let second_line = lines.next().context("bad input")?.trim();

    let mut corridors: Vec<_> = corridor_line.chars().enumerate()
        .filter(|(_ix, ch)| *ch == 'A' || *ch == 'B' || *ch == 'C' || *ch == 'D')
        .map(|(ix, ch)| ((ix - 1, 0), ch))
        .collect();

    let first_liners: Vec<_> = first_line
        .chars()
        .enumerate()
        .filter(|(_ix, ch)| *ch == 'A' || *ch == 'B' || *ch == 'C' || *ch == 'D')
        .map(|(ix, ch)| ((ix - 1, 1), ch))
        .collect();
    let second_liners: Vec<_> = second_line
        .chars()
        .enumerate()
        .filter(|(_ix, ch)| *ch == 'A' || *ch == 'B' || *ch == 'C' || *ch == 'D')
        .map(|(ix, ch)| ((ix + 1, 2), ch))
        .collect();

        corridors.extend(first_liners);
        corridors.extend(second_liners);
    Ok(corridors)
}

fn to_map(v: Vec<Amphypod>) -> HashMap<(usize, usize), char> {
    v.into_iter().collect::<HashMap<_, _>>()
}

fn to_vec(m: &HashMap<(usize, usize), char>) -> Vec<Amphypod> {
    let mut v : Vec<Amphypod> = m.iter().map(|(&k, &v)| (k, v)).collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {

        let s = "  #############
                        #...........#
                        ###D#B#A#C###
                          #C#A#D#B#
                          #########";

        let res = parse(s).unwrap();

        let expected = 
            vec![((2, 1), 'D'), ((4, 1), 'B'), ((6, 1), 'A'), ((8, 1), 'C'),
                 ((2, 2), 'C'), ((4, 2), 'A'), ((6, 2), 'D'), ((8, 2), 'B')];

        assert_eq!(res, expected);
    }

    #[test]
    fn test_inputs_01() {
        let s = "  #############
                        #.........A.#
                        ###.#B#C#D###
                          #A#B#C#D#
                          #########";

        let input = parse(s).unwrap();

        let mut state = to_map(input);

        let res = solve_01(&mut state, 2);

        assert_eq!(res, 8);

        let s = "  #############
                        #.....D.D.A.#
                        ###.#B#C#.###
                          #A#B#C#.#
                          #########";

        let input = parse(s).unwrap();

        let mut state = to_map(input);

        let res = solve_01(&mut state, 2);

        assert_eq!(res, 7008);

        let s = "  #############
                        #.....D.....#
                        ###.#B#C#D###
                          #A#B#C#A#
                          #########";

        let input = parse(s).unwrap();

        let mut state = to_map(input);

        let res = solve_01(&mut state, 2);

        assert_eq!(res, 9011);

        let s = "  #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########";

        let input = parse(s).unwrap();

        let mut state = to_map(input);

        let res = solve_01(&mut state, 2);

        assert_eq!(res, 12521);
    }

    #[test]
    fn test_moves_to_siderooms() {
        let s = "  #############
                        #.A.........#
                        ###.#C#B#D###
                          #A#D#C#B#
                          #########";

        let state = to_map(parse(s).unwrap());

        let moves = possible_moves(&state, (1, 0), 2);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].0, (2, 1));

        let s = "  #############
                        #.A.A.......#
                        ###.#C#B#D###
                          #.#D#C#B#
                          #########";

        let state = to_map(parse(s).unwrap());

        let moves = possible_moves(&state, (1, 0), 2);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].0, (2, 2));

        let s = "  #############
                        #.A.........#
                        ###.#C#A#D###
                          #B#D#C#B#
                          #########";

        let state = to_map(parse(s).unwrap());

        let moves = possible_moves(&state, (1, 0), 2);
        assert_eq!(moves.len(), 0);

        let s = "  #############
                        #...B.....A.#
                        ###.#C#A#D###
                          #.#D#C#B#
                          #########";

        let state = to_map(parse(s).unwrap());

        let moves = possible_moves(&state, (9, 0), 2);
        assert_eq!(moves.len(), 0);

    }

    #[test]
    fn test_moves_to_corridor() {
        let s = "  #############
                        #...........#
                        ###A#C#B#D###
                          #A#D#C#B#
                          #########";

        let state = to_map(parse(s).unwrap());

        // we do not want to move A because A's are finished 
        let moves = possible_moves(&state, (2, 1), 2);
        assert_eq!(moves.len(), 0);

        // cannot move D from second room because C is bloking
        let moves = possible_moves(&state, (4, 2), 2);
        assert_eq!(moves.len(), 0);

        // cannot move D from second room because C is bloking
        let moves = possible_moves(&state, (4, 2), 2);
        assert_eq!(moves.len(), 0);

        // can move D from 4th row, even though it is in the right one because there is a B under it
        let moves = possible_moves(&state, (8, 1), 2);
        assert_eq!(moves.len(), 7);

        let s = "  #############
                        #.....C.A...#
                        ###.#C#.#D###
                          #A#D#B#B#
                          #########";

        let state = to_map(parse(s).unwrap());

        // B from third has nowhere to move
        let moves = possible_moves(&state, (6, 2), 2);
        assert_eq!(moves.len(), 0);

        // But C from second can, though to the left only!
        let moves = possible_moves(&state, (4, 1), 2);
        assert_eq!(moves.len(), 3);
    }
}
