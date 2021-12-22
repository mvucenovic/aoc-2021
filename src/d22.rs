use anyhow::Context;
use std::collections::HashSet;

pub fn part_01() -> anyhow::Result<usize> {
    let inputs = inputs()?;

    let mut lighted = HashSet::new();

    for (light_on, (x0, x1), (y0, y1), (z0, z1)) in inputs {
        if (x0 > 50 && x1 > 50) || (x0 < -50 && x1 < -50) {
            continue;
        }

        if (y0 > 50 && y1 > 50) || (y0 < -50 && y1 < -50) {
            continue;
        }

        if (z0 > 50 && z1 > 50) || (z0 < -50 && z1 < -50) {
            continue;
        }

        for ix in x0..=x1 {
            for iy in y0..=y1 {
                for iz in z0..=z1 {
                    if ix.abs() > 50 && iy.abs() > 50 && iz.abs() > 50 {
                        continue;
                    }

                    if light_on {
                        lighted.insert((ix, iy, iz));
                    } else {
                        lighted.remove(&(ix, iy, iz));
                    }
                }
            }
        }
    }

    Ok(lighted.len())
}

pub fn part_02() -> anyhow::Result<u64> {
    let inputs = inputs()?;

    let mut volume = 0;

    for (ix, input) in inputs.iter().enumerate() {
        if input.0 == false {
            continue;
        }

        volume += unique_volume(input, &inputs[ix + 1..]);
    }

    Ok(volume) // too low 1261788327439723 / too high 1525886909356651
}

fn unique_volume(c: &Cuboid, others: &[Cuboid]) -> u64 {
    let intersection_cubes = others
        .iter()
        .filter_map(|other| intersect(c, other))
        .collect::<Vec<_>>();

    // we must remove intersections from intersected cubes
    let unique_intersecting_volume: u64 = intersection_cubes
        .iter()
        .enumerate()
        .map(|(ix, c)| unique_volume(c, &intersection_cubes[ix + 1..]))
        .sum();
    v(c) - unique_intersecting_volume
}

type Cuboid = (bool, (i32, i32), (i32, i32), (i32, i32));

use std::cmp::{max, min};

fn have_intersect(c0: &Cuboid, c1: &Cuboid) -> bool {
    let (_, (c0x0, c0x1), (c0y0, c0y1), (c0z0, c0z1)) = c0;
    let (_, (c1x0, c1x1), (c1y0, c1y1), (c1z0, c1z1)) = c1;

    c0x0 < c1x1 && c0x1 > c1x0 && c0y0 < c1y1 && c0y1 > c1y0 && c0z0 < c1z1 && c0z1 > c1z0
}

fn intersect(c0: &Cuboid, c1: &Cuboid) -> Option<Cuboid> {
    if !have_intersect(c0, c1) {
        return None;
    }

    let (_, (c0x0, c0x1), (c0y0, c0y1), (c0z0, c0z1)) = c0;
    let (_, (c1x0, c1x1), (c1y0, c1y1), (c1z0, c1z1)) = c1;

    let x0 = min(max(*c0x0, *c1x0), *c1x1); // max(*c0x0, *c1x0);
    let x1 = min(max(*c0x1, *c1x0), *c1x1); // min(*c0x1, *c1x1);

    let y0 = min(max(*c0y0, *c1y0), *c1y1);
    let y1 = min(max(*c0y1, *c1y0), *c1y1);

    let z0 = min(max(*c0z0, *c1z0), *c1z1);
    let z1 = min(max(*c0z1, *c1z0), *c1z1);

    Some((true, (x0, x1), (y0, y1), (z0, z1)))
}

fn v(c: &Cuboid) -> u64 {
    ((c.1 .1 - c.1 .0).abs() as u64 + 1)
        * ((c.2 .1 - c.2 .0).abs() as u64 + 1)
        * ((c.3 .1 - c.3 .0).abs() as u64 + 1)
}

fn inputs() -> anyhow::Result<Vec<Cuboid>> {
    let input_string =
        std::fs::read_to_string("inputs/22_input.txt").context("Error while reading input")?;

    parse(&input_string)
}

fn parse(s: &str) -> anyhow::Result<Vec<Cuboid>> {
    let mut res = vec![];
    for line in s.lines() {
        let mut split = line.trim().split(" ");
        let instruction_str = split.next().context("Bad input line")?.trim();

        let ins = if instruction_str == "on" { true } else { false };
        let coords = split.next().context("Bad input line")?.trim();

        // x=8088..31780,y=59042..84353,z=-35793..-10401
        let re =
            regex::Regex::new(r"x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

        let caps = re.captures(coords).context("Bad format for coords")?;

        let x = (caps[1].parse::<i32>()?, caps[2].parse::<i32>()?);
        let y = (caps[3].parse::<i32>()?, caps[4].parse::<i32>()?);
        let z = (caps[5].parse::<i32>()?, caps[6].parse::<i32>()?);

        res.push((ins, x, y, z))
    }
    Ok(res)
}
