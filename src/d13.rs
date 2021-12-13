use anyhow::Context;
use std::collections::HashSet;

pub fn part_01() -> anyhow::Result<usize> {
    let (mut dots, folds) = inputs()?;

    let fold = folds.into_iter().next().unwrap();

    do_fold(fold, &mut dots);

    Ok(dots.len())
}

pub fn part_02() -> anyhow::Result<()> {
    let (mut dots, folds) = inputs()?;

    for fold in folds {
        do_fold(fold, &mut dots);
    }

    print_dots(&dots);

    Ok(())
}

fn do_fold(fold: (isize, isize), dots: &mut HashSet<(isize, isize)>) {
    if fold.0 > 0 {
        let x_fold = fold.0;
        let starting_dots = dots.iter().copied().collect::<Vec<_>>();
        for dot in starting_dots {
            if dot.0 > x_fold {
                dots.remove(&dot);
                dots.insert((2 * x_fold - dot.0, dot.1));
            }
        }
    } else {
        let y_fold = fold.1;
        let starting_dots = dots.iter().copied().collect::<Vec<_>>();
        for dot in starting_dots {
            if dot.1 > y_fold {
                dots.remove(&dot);
                dots.insert((dot.0, 2 * y_fold - dot.1));
            }
        }
    }
}

fn print_dots(dots: &HashSet<(isize, isize)>) {
    let max_x = dots.iter().map(|d| d.0).max().unwrap();
    let max_y = dots.iter().map(|d| d.1).max().unwrap();

    for y in 0..=max_y {
        let mut line = String::new();
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push('.')
            }
        }
        println!("{}", line);
    }
}

fn inputs() -> anyhow::Result<(HashSet<(isize, isize)>, Vec<(isize, isize)>)> {
    let input_string =
        std::fs::read_to_string("inputs/13_input.txt").context("Error while reading input")?;

    parse(&input_string)
}

fn parse(input: &str) -> anyhow::Result<(HashSet<(isize, isize)>, Vec<(isize, isize)>)> {
    let mut sections_split = input.split("\n\n");

    let dots_section = sections_split.next().context("Missing section with dots")?;
    let folds_section = sections_split
        .next()
        .context("Missing section with folding instructions")?;

    let dots = dots_section
        .lines()
        .map(|l| {
            let mut s = l.split(",");
            (
                s.next().unwrap().parse::<isize>().unwrap(),
                s.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    let folds = folds_section
        .lines()
        .map(|l| {
            let mut split = l.split("=");
            let axis_char = split.next().unwrap().chars().last().unwrap();

            let number = split.next().unwrap().parse::<isize>().unwrap();
            if axis_char == 'x' {
                (number, 0)
            } else {
                (0, number)
            }
        })
        .collect::<Vec<_>>();

    Ok((dots, folds))
}
