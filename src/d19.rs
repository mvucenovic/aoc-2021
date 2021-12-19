#![allow(dead_code)]

use anyhow::Context;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_01() -> anyhow::Result<usize> {
    let mut inputs = inputs()?;
    let mut set = inputs.remove(0);

    'outer: while inputs.len() > 0 {
        for (ix, input) in inputs.iter().enumerate() {
            if let Some(res) = try_fit_par(&set, input, 12) {
                set = res.0;
                inputs.remove(ix);
                continue 'outer;
            }
        }
        panic!("No pair found!");
    }

    Ok(set.len())
}

pub fn part_02() -> anyhow::Result<i32> {
    let mut inputs = inputs()?;
    let mut set = inputs.remove(0);

    let mut positions = vec![(0, 0, 0)];

    'outer: while inputs.len() > 0 {
        for (ix, input) in inputs.iter().enumerate() {
            if let Some(res) = try_fit_par(&set, input, 12) {
                set = res.0;
                inputs.remove(ix);
                positions.push(res.1);
                continue 'outer;
            }
        }
        panic!("No pair found!");
    }

    let res = positions
        .into_iter()
        .permutations(2)
        .map(|perm| manhatten_distance(&perm[0], &perm[1]))
        .max();

    Ok(res.unwrap())
}

fn manhatten_distance(c1: &Coord, c2: &Coord) -> i32 {
    (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs() + (c1.2 - c2.2).abs()
}

const ROTATIONS: [[[i32; 3]; 3]; 24] = [
    // https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm

    // 0 - normal
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    // 1 - Rotate 0,90,0
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    // 2 - rotate 0,180,0
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    // 3 - rotate 0,270,0
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    // 4 - rotate 0,0,90
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    // 5 - rotate 0,90,90
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    // 6 - rotate 0,180,90
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    // 7 - rotate 0,270,90
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    // 8 - rotate 0,0,270
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    // 9 - rotate 0,90,270
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    // 10 - rotate 0,180,270
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    // 11 - rotate 0,270,270
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    // 12 - rotate 90,0,0
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    // 13 - rotate 90,90,0
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    // 14 - rotate 90,180,0
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    // 15 - rotate 90,270,0
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    // 16 - rotate 180,0,0
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    // 17 - rotate 180,90,0
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    // 18 - rotate 180,180,0
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    // 19 - rotate 180,270,0
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    // 20 - rotate 270,0,0
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    // 21 - rotate 270,90,0
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    // 22 - rotate 270,180,0
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    // 23 - rotate 270,270,0
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
];

fn rotate(c: &Coord, rotation_matrix: &[[i32; 3]; 3]) -> Coord {
    let new_x =
        rotation_matrix[0][0] * c.0 + rotation_matrix[0][1] * c.1 + rotation_matrix[0][2] * c.2;
    let new_y =
        rotation_matrix[1][0] * c.0 + rotation_matrix[1][1] * c.1 + rotation_matrix[1][2] * c.2;
    let new_z =
        rotation_matrix[2][0] * c.0 + rotation_matrix[2][1] * c.1 + rotation_matrix[2][2] * c.2;
    (new_x, new_y, new_z)
}

fn unrotate(c: &Coord, rotation_matrix: &[[i32; 3]; 3]) -> Coord {
    // transpose rotation matrix
    let new_x =
        rotation_matrix[0][0] * c.0 + rotation_matrix[1][0] * c.1 + rotation_matrix[2][0] * c.2;
    let new_y =
        rotation_matrix[0][1] * c.0 + rotation_matrix[1][1] * c.1 + rotation_matrix[2][1] * c.2;
    let new_z =
        rotation_matrix[0][2] * c.0 + rotation_matrix[1][2] * c.1 + rotation_matrix[2][2] * c.2;
    (new_x, new_y, new_z)
}

const SONAR_DISTANCE: i32 = 1000;

fn is_possible_candidate(delta: &Coord) -> bool {
    delta.0.abs() <= SONAR_DISTANCE * 2
        && delta.1.abs() <= SONAR_DISTANCE * 2
        && delta.2.abs() <= SONAR_DISTANCE * 2
}

fn try_fit_par(s1: &Sonar, s2: &Sonar, number: usize) -> Option<(HashSet<Coord>, Coord, usize)> {
    for rot in ROTATIONS {
        for c2 in s2 {
            let c2_unrot = unrotate(c2, &rot);
            for c1 in s1 {
                let delta = sub_coords(&c2_unrot, c1);

                // if !is_possible_candidate(&delta) {
                //     continue;
                // }

                let (coords_set, count) =
                    fit_after_rotation_and_transposition(s1, s2, &rot, &delta);

                if count >= number {
                    // this is a solution
                    return Some((coords_set, minus_vector(&delta), count));
                }
            }
        }
    }
    return None;
}

fn fit_after_rotation_and_transposition(
    s1: &Sonar,
    s2: &Sonar,
    rot: &[[i32; 3]; 3],
    delta: &Coord,
) -> (Sonar, usize) {
    let mut count = 0;
    let mut coords_set = s1.clone();
    for c2 in s2 {
        let c2_unrot = unrotate(c2, rot);
        let c2_unrot_and_transposed = sub_coords(&c2_unrot, delta);
        if coords_set.contains(&c2_unrot_and_transposed) {
            count += 1;
        } else {
            coords_set.insert(c2_unrot_and_transposed);
        }
    }

    (coords_set, count)
}

fn add_coords(c1: &Coord, c2: &Coord) -> Coord {
    (c1.0 + c2.0, c1.1 + c2.1, c1.2 + c2.2)
}

fn sub_coords(c1: &Coord, c2: &Coord) -> Coord {
    (c1.0 - c2.0, c1.1 - c2.1, c1.2 - c2.2)
}

fn minus_vector(c1: &Coord) -> Coord {
    (-1 * c1.0, -1 * c1.1, -1 * c1.2)
}

type Coord = (i32, i32, i32);
type Sonar = HashSet<Coord>;

fn inputs() -> anyhow::Result<Vec<Sonar>> {
    let input_string =
        std::fs::read_to_string("inputs/19_input.txt").context("Error while reading input")?;

    Ok(parse(&input_string))
}

fn parse(s: &str) -> Vec<Sonar> {
    let split_by_scanner = s.split("\n\n");
    let mut result = vec![];

    for scanner in split_by_scanner {
        let scanner_coords = parse_one(scanner);
        result.push(scanner_coords);
    }
    result
}

fn parse_one(s: &str) -> Sonar {
    s.lines()
        .skip(1)
        .map(|l| {
            let (x, y, z) = l.trim().split(',').collect_tuple().unwrap();
            (
                x.parse::<i32>().unwrap(),
                y.parse::<i32>().unwrap(),
                z.parse::<i32>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rotate_unrotate() {
        let dot = (3, 5, 7);
        for rot in ROTATIONS.iter() {
            let rotated = rotate(&dot, &rot);
            let res = unrotate(&rotated, &rot);
            assert_eq!(dot, res);
        }
    }

    #[test]
    fn test_first_input() {
        let s1 = parse_one(
            "--- scanner 0 ---
        0,2,0
        4,1,0
        3,3,0",
        );

        let s2 = parse_one(
            "--- scanner 1 ---
        -1,-1,0
        -5,0,0
        -2,1,0",
        );

        let res = try_fit_par(&s1, &s2, 3);

        let (set, _, n) = res.unwrap();
        assert_eq!(n, 3);
        assert_eq!(s1, set);
    }

    #[test]
    fn test_second_input() {
        let resulting = resulting();

        let s0 = s0();

        let s1 = s1();

        let res = try_fit_par(&s0, &s1, 12);
        let (set, delta, n) = res.unwrap();
        assert_eq!(n, 12);

        assert_eq!(delta, (68, -1246, -43));
        assert_eq!(set.len(), 38);

        println!("{:?}", set.difference(&resulting));
        assert!(resulting.is_superset(&set));

        let s4 = s4();

        let res = try_fit_par(&set, &s4, 12);
        let (set, delta, n) = res.unwrap();
        assert_eq!(n, 12);

        assert_eq!(delta, (-20, -1133, 1061));
        assert_eq!(set.len(), 52);

        assert!(resulting.is_superset(&set));

        let s2 = s2();

        let res = try_fit_par(&set, &s2, 12);

        let (set, delta, n) = res.unwrap();
        assert_eq!(n, 12);

        assert_eq!(delta, (1105, -1205, 1229));
        assert_eq!(set.len(), 66);

        assert!(resulting.is_superset(&set));

        let s3 = s3();
        let res = try_fit_par(&set, &s3, 12);

        let (set, delta, n) = res.unwrap();
        assert_eq!(n, 12);

        assert_eq!(delta, (-92, -2380, -20));
    }

    #[test]
    fn try_with_resulting() {
        let resulting = resulting();

        let s3 = s3();

        let res = try_fit_par(&resulting, &s3, 12);
        res.unwrap();
    }

    #[test]
    fn debug_test_case() {
        let difference: Sonar = [
            (-430, -3130, 366),
            (846, -3110, -434),
            (-470, -3283, 303),
            (346, -2985, 342),
            (776, -3184, -501),
            (366, -3059, 397),
            (-654, -3158, -753),
            (-706, -3180, -659),
            (12, -2351, -103),
            (377, -2827, 367),
            (-697, -3072, -689),
            (-620, -3212, 371),
            (686, -3108, -505),
        ]
        .into();
        let s3_pos = minus_vector(&(-92, -2380, -20));

        let mut s3 = s3();

        'outer: for one in difference.iter() {
            for (ix, rot) in ROTATIONS.iter().enumerate() {
                let diff = add_coords(one, &s3_pos);
                let rot = rotate(&diff, &rot);

                if s3.contains(&rot) {
                    println!("{} - Contains {:?}", ix, rot);
                    s3.remove(&rot);
                    continue 'outer;
                }
            }
            panic!("All coords must be contained!")
        }
        println!("{:?}", s3);

        let res1 = s3
            .iter()
            .map(|c| sub_coords(&unrotate(c, &ROTATIONS[2]), &s3_pos))
            .collect::<Vec<_>>();
        println!("res1 = {:?}", res1);
    }

    fn resulting() -> Sonar {
        parse_one(
            "---RESULT----
        -892,524,684
        -876,649,763
        -838,591,734
        -789,900,-551
        -739,-1745,668
        -706,-3180,-659
        -697,-3072,-689
        -689,845,-530
        -687,-1600,576
        -661,-816,-575
        -654,-3158,-753
        -635,-1737,486
        -631,-672,1502
        -624,-1620,1868
        -620,-3212,371
        -618,-824,-621
        -612,-1695,1788
        -601,-1648,-643
        -584,868,-557
        -537,-823,-458
        -532,-1715,1894
        -518,-1681,-600
        -499,-1607,-770
        -485,-357,347
        -470,-3283,303
        -456,-621,1527
        -447,-329,318
        -430,-3130,366
        -413,-627,1469
        -345,-311,381
        -36,-1284,1171
        -27,-1108,-65
        7,-33,-71
        12,-2351,-103
        26,-1119,1091
        346,-2985,342
        366,-3059,397
        377,-2827,367
        390,-675,-793
        396,-1931,-563
        404,-588,-901
        408,-1815,803
        423,-701,434
        432,-2009,850
        443,580,662
        455,729,728
        456,-540,1869
        459,-707,401
        465,-695,1988
        474,580,667
        496,-1584,1900
        497,-1838,-617
        527,-524,1933
        528,-643,409
        534,-1912,768
        544,-627,-890
        553,345,-567
        564,392,-477
        568,-2007,-577
        605,-1665,1952
        612,-1593,1893
        630,319,-379
        686,-3108,-505
        776,-3184,-501
        846,-3110,-434
        1135,-1161,1235
        1243,-1093,1063
        1660,-552,429
        1693,-557,386
        1735,-437,1738
        1749,-1800,1813
        1772,-405,1572
        1776,-675,371
        1779,-442,1789
        1780,-1548,337
        1786,-1538,337
        1847,-1591,415
        1889,-1729,1762
        1994,-1805,1792",
        )
    }

    fn s0() -> Sonar {
        parse_one(
            "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401",
        )
    }

    fn s1() -> Sonar {
        parse_one(
            "--- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390",
        )
    }

    fn s4() -> Sonar {
        parse_one(
            "--- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14",
        )
    }

    fn s2() -> Sonar {
        parse_one(
            "--- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562",
        )
    }

    fn s3() -> Sonar {
        parse_one(
            "--- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596",
        )
    }
}
