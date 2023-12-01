use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::recognize,
    multi::{many0, many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::{
    collections::{BTreeSet, LinkedList},
    ops::Sub,
};

pub fn solve(problem: &str) -> (u64, u64) {
    let scanners = parse_scanners(problem);
    let calibrated = calibrated_scanners(&scanners);
    (solve1(&calibrated), solve2(&calibrated))
}

fn solve1(calibrated_scanners: &[Scanner]) -> u64 {
    let deduped_beacons = dedup_beacon_locations(calibrated_scanners);
    deduped_beacons.len() as u64
}

fn solve2(calibrated_scanners: &[Scanner]) -> u64 {
    calibrated_scanners
        .iter()
        .tuple_combinations()
        .map(|(s1, s2)| s1.center.manhattan_distance_from(&s2.center))
        .max()
        .unwrap()
}

fn parse_scanners(input: &str) -> Vec<Scanner> {
    let (_input, scanners) = many0(Scanner::parse)(input).unwrap();
    scanners
}

fn calibrated_scanners(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut calibrated = vec![scanners[0].clone()];
    let mut to_calibrate: LinkedList<_> = scanners[1..].iter().cloned().map(|s| (0, s)).collect();

    while !to_calibrate.is_empty() {
        let (next_idx_to_calibrate_against, uncalibrated) = to_calibrate.pop_front().unwrap();
        let to_calibrate_against = &calibrated[next_idx_to_calibrate_against];
        let maybe_calibrated = uncalibrated.calibrated_against(to_calibrate_against);
        match maybe_calibrated {
            None => to_calibrate.push_back((next_idx_to_calibrate_against + 1, uncalibrated)),
            Some(scanner) => calibrated.push(scanner),
        }
    }

    calibrated
}

fn dedup_beacon_locations(scanners: &[Scanner]) -> BTreeSet<Point> {
    scanners
        .iter()
        .flat_map(|s| s.beacon_locations.clone())
        .collect()
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Scanner {
    center: Point,
    beacon_locations: BTreeSet<Point>,
}

impl Scanner {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, coords) = preceded(
            tuple((tag("--- scanner "), many1(digit1), tag(" ---"))),
            terminated(many1(Point::parse), many1(tag("\n"))),
        )(input)?;

        Ok((
            input,
            Self {
                center: Point { x: 0, y: 0, z: 0 },
                beacon_locations: coords.into_iter().collect(),
            },
        ))
    }

    fn all_permutations(&self) -> Vec<Self> {
        // make each side face front
        let front = self.clone();
        let right = front.rotated_about(&Axis::Z);
        let back = right.rotated_about(&Axis::Z);
        let left = back.rotated_about(&Axis::Z);
        let top = front.rotated_about(&Axis::Y);
        let bottom = top.rotated_about(&Axis::Y).rotated_about(&Axis::Y);
        [front, right, back, left, top, bottom]
            .into_iter()
            .flat_map(|scanner| {
                let rot_0 = scanner;
                let rot_1 = rot_0.rotated_about(&Axis::X);
                let rot_2 = rot_1.rotated_about(&Axis::X);
                let rot_3 = rot_2.rotated_about(&Axis::X);
                [rot_0, rot_1, rot_2, rot_3]
            })
            .collect()
    }
    fn calibrated_against(&self, other: &Scanner) -> Option<Self> {
        self.all_permutations()
            .iter()
            .filter_map(|scanner| scanner.calibrated_translational_against(other))
            .at_most_one()
            .expect("there should not be more than one match")
    }

    fn calibrated_translational_against(&self, other: &Scanner) -> Option<Scanner> {
        self.beacon_locations
            .iter()
            .cartesian_product(&other.beacon_locations)
            .filter_map(|(self_point, other_point)| {
                let diff = other_point - self_point;
                let translated = self.translate(&diff);
                let intersection = translated
                    .beacon_locations
                    .intersection(&other.beacon_locations)
                    .collect_vec();

                if intersection.len() >= 12 {
                    return Some(translated);
                }
                None
            })
            .next()
    }

    /// 90 degrees counter-clockwise
    fn rotated_about(&self, axis: &Axis) -> Self {
        Self {
            center: self.center.clone(),
            beacon_locations: self
                .beacon_locations
                .iter()
                .cloned()
                .map(|p| p.rotated_about(axis))
                .collect(),
        }
    }

    fn translate(&self, vector3: &Vector3) -> Self {
        Self {
            center: self.center.plus(vector3),
            beacon_locations: self
                .beacon_locations
                .iter()
                .cloned()
                .map(|p| p.plus(vector3))
                .collect(),
        }
    }
}

enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, nums): (_, Vec<&str>) = preceded(
            tag("\n"),
            separated_list1(tag(","), recognize(many1(alt((tag("-"), digit1))))),
        )(input)?;

        let (x, y, z) = nums.into_iter().collect_tuple().unwrap();

        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        let z = z.parse::<i32>().unwrap();

        Ok((input, Self { x, y, z }))
    }

    /// 90 degrees counter-clockwise
    fn rotated_about(&self, axis: &Axis) -> Self {
        match axis {
            Axis::X => Self {
                x: self.x,
                y: -self.z,
                z: self.y,
            },
            Axis::Y => Self {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            Axis::Z => Self {
                x: -self.y,
                y: self.x,
                z: self.z,
            },
        }
    }

    fn plus(&self, vector3: &Vector3) -> Self {
        Self {
            x: self.x + vector3.x,
            y: self.y + vector3.y,
            z: self.z + vector3.z,
        }
    }

    fn manhattan_distance_from(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)) as u64
    }
}

impl Sub for &Point {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Hash, Clone)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_point_parse() {
    let point_str = "\n404,-588,-901";
    let (_, point) = Point::parse(point_str).unwrap();
    assert_eq!(
        Point {
            x: 404,
            y: -588,
            z: -901,
        },
        point
    )
}

#[test]
fn test_scanner_parse() {
    let scanner_str = r#"--- scanner 0 ---
404,-588,-901

---"#;
    let (_, scanner) = Scanner::parse(scanner_str).unwrap();
    assert_eq!(
        Scanner {
            center: Point { x: 0, y: 0, z: 0 },
            beacon_locations: [Point {
                x: 404,
                y: -588,
                z: -901,
            }]
            .into()
        },
        scanner
    )
}

#[test]
fn test_scanners_parse() {
    let scanners_str = r#"--- scanner 0 ---
404,-588,-901

--- scanner 100 ---
0,1,2
"#;
    let scanners = parse_scanners(scanners_str);
    assert_eq!(
        vec![
            Scanner {
                center: Point { x: 0, y: 0, z: 0 },
                beacon_locations: [Point {
                    x: 404,
                    y: -588,
                    z: -901,
                }]
                .into()
            },
            Scanner {
                center: Point { x: 0, y: 0, z: 0 },
                beacon_locations: [Point { x: 0, y: 1, z: 2 }].into()
            }
        ],
        scanners
    )
}

#[test]
fn test_point_rotation() {
    let p = Point { x: 1, y: 1, z: 1 };

    assert_eq!(Point { x: 1, y: -1, z: 1 }, p.rotated_about(&Axis::X));
}

#[test]
fn test_scanner_permutations() {
    let scanner = Scanner {
        center: Point { x: 0, y: 0, z: 0 },
        beacon_locations: BTreeSet::from([Point { x: 1, y: 2, z: 3 }]),
    };

    let actual = scanner
        .all_permutations()
        .into_iter()
        .sorted()
        .collect_vec();
    let center_zero = Point { x: 0, y: 0, z: 0 };
    assert_eq!(
        [
            // front
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 1, y: 2, z: 3 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 1, y: -3, z: 2 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 1, y: -2, z: -3 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 1, y: 3, z: -2 }]),
            },
            // right
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -2, y: 1, z: 3 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -2, y: -3, z: 1 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point {
                    x: -2,
                    y: -1,
                    z: -3
                }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -2, y: 3, z: -1 }]),
            },
            // left
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 2, y: -1, z: 3 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 2, y: -3, z: -1 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 2, y: 1, z: -3 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 2, y: 3, z: 1 }]),
            },
            // back
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -1, y: -2, z: 3 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point {
                    x: -1,
                    y: -3,
                    z: -2
                }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -1, y: 2, z: -3 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -1, y: 3, z: 2 }]),
            },
            // top
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 3, y: 2, z: -1 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 3, y: 1, z: 2 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 3, y: -2, z: 1 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: 3, y: -1, z: -2 }]),
            },
            // bottom
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -3, y: 2, z: 1 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -3, y: -1, z: 2 }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point {
                    x: -3,
                    y: -2,
                    z: -1
                }]),
            },
            Scanner {
                center: center_zero.clone(),
                beacon_locations: BTreeSet::from([Point { x: -3, y: 1, z: -2 }]),
            },
        ]
        .into_iter()
        .sorted()
        .collect_vec(),
        actual
    );
}

#[test]
fn points_intersect() {
    let a = BTreeSet::from([
        Point {
            x: -892,
            y: 524,
            z: 684,
        },
        Point {
            x: -876,
            y: 649,
            z: 763,
        },
        Point {
            x: -838,
            y: 591,
            z: 734,
        },
        Point {
            x: -789,
            y: 900,
            z: -551,
        },
        Point {
            x: -689,
            y: 845,
            z: -530,
        },
        Point {
            x: -661,
            y: -816,
            z: -575,
        },
        Point {
            x: -618,
            y: -824,
            z: -621,
        },
        Point {
            x: -584,
            y: 868,
            z: -557,
        },
        Point {
            x: -537,
            y: -823,
            z: -458,
        },
        Point {
            x: -485,
            y: -357,
            z: 347,
        },
        Point {
            x: -447,
            y: -329,
            z: 318,
        },
        Point {
            x: -345,
            y: -311,
            z: 381,
        },
        Point {
            x: 7,
            y: -33,
            z: -71,
        },
        Point {
            x: 390,
            y: -675,
            z: -793,
        },
        Point {
            x: 404,
            y: -588,
            z: -901,
        },
        Point {
            x: 423,
            y: -701,
            z: 434,
        },
        Point {
            x: 443,
            y: 580,
            z: 662,
        },
        Point {
            x: 455,
            y: 729,
            z: 728,
        },
        Point {
            x: 459,
            y: -707,
            z: 401,
        },
        Point {
            x: 474,
            y: 580,
            z: 667,
        },
        Point {
            x: 528,
            y: -643,
            z: 409,
        },
        Point {
            x: 544,
            y: -627,
            z: -890,
        },
        Point {
            x: 553,
            y: 345,
            z: -567,
        },
        Point {
            x: 564,
            y: 392,
            z: -477,
        },
        Point {
            x: 630,
            y: 319,
            z: -379,
        },
    ]);
    let b = BTreeSet::from([
        Point {
            x: -892,
            y: 524,
            z: 684,
        },
        Point {
            x: -868,
            y: 1904,
            z: 997,
        },
        Point {
            x: -858,
            y: 619,
            z: -661,
        },
        Point {
            x: -852,
            y: 1888,
            z: -302,
        },
        Point {
            x: -821,
            y: 693,
            z: 724,
        },
        Point {
            x: -783,
            y: 1824,
            z: -294,
        },
        Point {
            x: -756,
            y: 522,
            z: -743,
        },
        Point {
            x: -747,
            y: 1830,
            z: -327,
        },
        Point {
            x: -732,
            y: 716,
            z: -696,
        },
        Point {
            x: -728,
            y: 1943,
            z: 1008,
        },
        Point {
            x: -720,
            y: 600,
            z: 670,
        },
        Point {
            x: -714,
            y: 1856,
            z: 900,
        },
        Point {
            x: -297,
            y: 1423,
            z: 172,
        },
        Point {
            x: 21,
            y: 2220,
            z: -274,
        },
        Point {
            x: 123,
            y: 2202,
            z: -211,
        },
        Point {
            x: 161,
            y: 2174,
            z: -240,
        },
        Point {
            x: 175,
            y: 924,
            z: 877,
        },
        Point {
            x: 194,
            y: 850,
            z: 707,
        },
        Point {
            x: 213,
            y: 1708,
            z: 565,
        },
        Point {
            x: 277,
            y: 883,
            z: 750,
        },
        Point {
            x: 294,
            y: 1707,
            z: 728,
        },
        Point {
            x: 311,
            y: 794,
            z: -379,
        },
        Point {
            x: 337,
            y: 1715,
            z: 682,
        },
        Point {
            x: 363,
            y: 931,
            z: -469,
        },
        Point {
            x: 415,
            y: 786,
            z: -561,
        },
    ]);
    let intersection = a.intersection(&b).collect_vec();
    assert_eq!(
        vec![&Point {
            x: -892,
            y: 524,
            z: 684
        }],
        intersection
    )
}
