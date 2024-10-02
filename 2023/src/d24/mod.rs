use itertools::Itertools;
use num::{BigRational, FromPrimitive, ToPrimitive, Zero, Signed};
use std::ops::{Add, Mul, Sub};

pub fn solve(problem: &str) -> (usize, usize) {
    let hailstones = problem.lines().map(Hailstone::parse).collect_vec();
    (solve1(&hailstones), solve2(&hailstones))
}

fn solve1(hailstones: &[Hailstone]) -> usize {
    let min_pos = BigRational::from_usize(200000000000000).unwrap();
    let max_pos = BigRational::from_usize(400000000000000).unwrap();
    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(h1, h2)| {
            let maybe_intersection = flat_intersection(h1, h2);
            let intersection = match maybe_intersection {
                Some(i) => i,
                None => return false,
            };

            if intersection.x < min_pos || intersection.x > max_pos {
                return false;
            }

            if intersection.y < min_pos || intersection.y > max_pos {
                return false;
            }

            if happens_before_for_any(&intersection, &[h1, h2]) {
                return false;
            }

            true
        })
        .count()
}

fn solve2(hailstones: &[Hailstone]) -> usize {
    let range1 = if hailstones.len() <= 5 {
        -5_isize..5
    } else {
        130..180
    };

    let range2 = if hailstones.len() <= 5 {
        -5_isize..5
    } else {
        130..180
    };

    let range3 = if hailstones.len() <= 5 {
        -5_isize..5
    } else {
        230..280
    };

    let (flattened_position, direction) = (range1)
        .cartesian_product(range2)
        .cartesian_product(range3)
        .find_map(|((u, v), w)| {
            // search the full space of potential velocity vectors
            let vel_vector = Point3 {
                x: BigRational::from_isize(u).unwrap(),
                y: BigRational::from_isize(v).unwrap(),
                z: BigRational::from_isize(w).unwrap(),
            };

            // this might be where all the lines interect, if flattened against the above velocity vector
            let maybe_position = hailstones
                .iter()
                .map(|h| {
                    // this finds the velocity component perpendicular to the potential stone we are throwing
                    let velocity_rejection = h.velocity.reject_on(&vel_vector);
                    // this finds the position the hailstone would be if on the same plane as the origin, given the flattened velocity vector
                    let new_position = point_translated_along_vector_to_same_plane_as_point(
                        &h.position,
                        &vel_vector,
                        &Point3 {
                            x: BigRational::zero(),
                            y: BigRational::zero(),
                            z: BigRational::zero(),
                        },
                    );

                    Hailstone {
                        position: new_position,
                        velocity: velocity_rejection,
                    }
                })
                .tuple_combinations()
                .filter_map(|(h1, h2)| {
                    // trying to figure out where each pair of flattened lines intersect
                    let i = intersection(&h1, &h2);
                    if i == Intersection::Infinite {
                        // if they are the same line, then obviously they intersect
                        None
                    } else {
                        Some(i)
                    }
                }).all_equal_value(); // if they are all the same intersection point, then we found a stone that can intersect all the paths

            match maybe_position {
                Ok(Intersection::One(point)) => Some((point, vel_vector)),
                _ => {
                    None
                },
            }
        })
        .unwrap();

    let new_hailstone = Hailstone {
        position: flattened_position,
        velocity: direction.clone(),
    };

    // find where this new given line intersects another line
    let h1 = &hailstones[0];

    let intersect_1 = match intersection(h1, &new_hailstone) {
        Intersection::One(point3) => point3,
        _ => panic!("should have one intersection"),
    };
    
    // the time at which this hailstone intsersects the line
    let t_intersect_1 = ((&h1.position.x - &intersect_1.x) / &h1.velocity.x).abs();

    // where the stone needs to start to intersept the line
    let start_position = &intersect_1 - &(&direction * t_intersect_1);

    let x = start_position.x.to_integer().to_usize().unwrap();
    let y = start_position.y.to_integer().to_usize().unwrap();
    let z = start_position.z.to_integer().to_usize().unwrap();
    x + y + z
}

fn point_translated_along_vector_to_same_plane_as_point(
    starting_point: &Point3,
    normal: &Point3,
    destination_point: &Point3,
) -> Point3 {
    let displacement = starting_point - destination_point;
    let displacement_projection = displacement.project_on(normal);
    let new_point = starting_point - &displacement_projection;

    assert!(is_on_plane(&new_point, destination_point, normal));

    new_point
}

fn is_on_plane(point: &Point3, plane_point: &Point3, plane_normal: &Point3) -> bool {
    let a = &plane_normal.x;
    let b = &plane_normal.y;
    let c = &plane_normal.z;
    let d = &(a * &plane_point.x + b * &plane_point.y + c * &plane_point.z);

    &(a * &point.x + b * &point.y + c * &point.z) == d
}

fn intersection(h1: &Hailstone, h2: &Hailstone) -> Intersection {
    let x1 = &h1.position.x;
    let y1 = &h1.position.y;
    let z1 = &h1.position.z;
    let u1 = &h1.velocity.x;
    let v1 = &h1.velocity.y;
    let w1 = &h1.velocity.z;

    let x2 = &h2.position.x;
    let y2 = &h2.position.y;
    let z2 = &h2.position.z;
    let u2 = &h2.velocity.x;
    let v2 = &h2.velocity.y;
    let w2 = &h2.velocity.z;

    let parallel_xy = u1 * v2 == v1 * u2;
    let parallel_xz = u1 * w2 == w1 * u2;

    if parallel_xy && parallel_xz {
        let ax = v1 * w1 * (x2 - x1);
        let ay = u1 * w1 * (y2 - y1);
        let az = u1 * v1 * (z2 - z1);
        if ax == ay && ay == az {
            return Intersection::Infinite;
        }
        return Intersection::None;
    }

    let t2 = if !parallel_xy {
        &((u1 * (y1 - y2) + v1 * (x2 - x1)) / (u1 * v2 - v1 * u2))
    } else {
        &((u1 * (z1 - z2) + w1 * (x2 - x1)) / (u1 * w2 - w1 * u2))
    };

    let t1 = if u1 != &BigRational::zero() {
        &((x2 - x1 + t2 * u2) / u1)
    } else if v1 != &BigRational::zero() {
        &((y2 - y1 + t2 * v2) / v1)
    } else if w1 != &BigRational::zero() {
        &((z2 - z1 + t2 * w2) / w1)
    } else {
        return Intersection::None;
    };

    let x1_t1 = x1 + t1 * u1;
    let x2_t2 = x2 + t2 * u2;

    if x1_t1 != x2_t2 {
        return Intersection::None;
    }

    let y1_t1 = y1 + t1 * v1;
    let y2_t2 = y2 + t2 * v2;

    if y1_t1 != y2_t2 {
        return Intersection::None;
    }

    let z1_t1 = z1 + t1 * w1;
    let z2_t2 = z2 + t2 * w2;

    if z1_t1 != z2_t2 {
        return Intersection::None;
    }
    Intersection::One(Point3 {
        x: x1_t1,
        y: y1_t1,
        z: z1_t1,
    })
}

#[derive(Debug, PartialEq)]
enum Intersection {
    None,
    One(Point3),
    Infinite,
}

fn flat_intersection(h1: &Hailstone, h2: &Hailstone) -> Option<Point3> {
    let m1 = &h1.velocity.y / &h1.velocity.x;
    let m2 = &h2.velocity.y / &h2.velocity.x;

    if m1 == m2 {
        return None;
    }

    let x1 = &h1.position.x;
    let x2 = &h2.position.x;

    let y1 = &h1.position.y;
    let y2 = &h2.position.y;

    let intersection_x = (y2 - y1 + &m1 * x1 - &m2 * x2) / (&m1 - &m2);
    let intersection_y = &m1 * (&intersection_x - x1) + y1;

    Some(Point3 {
        x: intersection_x,
        y: intersection_y,
        z: BigRational::zero(),
    })
}

fn happens_before_for_any(point: &Point3, hailstones: &[&Hailstone]) -> bool {
    hailstones.iter().any(|hailstone| {
        let direction_of_point = point.x < hailstone.position.x;
        let direction_of_hailstone = hailstone.velocity.x < BigRational::zero();

        direction_of_point != direction_of_hailstone
    })
}

#[derive(Debug, PartialEq, Clone)]
struct Hailstone {
    position: Point3,
    velocity: Point3,
}

impl Hailstone {
    fn parse(input: &str) -> Self {
        let input = input.replace([',', '@'], "");
        let (p1, p2, p3, v1, v2, v3) = input.split_whitespace().collect_tuple().unwrap();
        Self {
            position: Point3 {
                x: p1.parse().unwrap(),
                y: p2.parse().unwrap(),
                z: p3.parse().unwrap(),
            },
            velocity: Point3 {
                x: v1.parse().unwrap(),
                y: v2.parse().unwrap(),
                z: v3.parse().unwrap(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Point3 {
    x: BigRational,
    y: BigRational,
    z: BigRational,
}

impl Point3 {
    fn project_on(&self, other: &Self) -> Self {
        other * (self.dot(other) / other.dot(other))
    }

    fn reject_on(&self, other: &Self) -> Self {
        let projection = self.project_on(other);
        let rejection = self - &projection;
        assert!(rejection.dot(other) == BigRational::zero());
        rejection
    }

    fn dot(&self, other: &Self) -> BigRational {
        &self.x * &other.x + &self.y * &other.y + &self.z * &other.z
    }
}

impl Sub<&Point3> for &Point3 {
    type Output = Point3;

    fn sub(self, rhs: &Point3) -> Self::Output {
        Point3 {
            x: &self.x - &rhs.x,
            y: &self.y - &rhs.y,
            z: &self.z - &rhs.z,
        }
    }
}

impl Mul<BigRational> for &Point3 {
    type Output = Point3;

    fn mul(self, rhs: BigRational) -> Self::Output {
        Point3 {
            x: &self.x * &rhs,
            y: &self.y * &rhs,
            z: &self.z * &rhs,
        }
    }
}

impl Add for &Point3 {
    type Output = Point3;

    fn add(self, rhs: &Point3) -> Self::Output {
        Point3 {
            x: &self.x + &rhs.x,
            y: &self.y + &rhs.y,
            z: &self.z + &rhs.z,
        }
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
