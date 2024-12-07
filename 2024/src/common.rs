use num_traits::Num;
use std::{
    cmp::Eq,
    env,
    fmt::Debug,
    fs::read_to_string,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
    time,
};

pub fn test<T: Debug + Eq, F: Fn(&str) -> T>(
    file_name: &str,
    module_path: &str,
    f: F,
    expected: T,
) {
    test_with_params(file_name, module_path, |s: &str, _: ()| f(s), (), expected);
}

pub fn test_with_params<P, T: Debug + Eq, F: Fn(&str, P) -> T>(
    file_name: &str,
    module_path: &str,
    f: F,
    params: P,
    expected: T,
) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let module_name = module_path.split("::").last().unwrap();
    let input_file = format!("{manifest_dir}/src/{module_name}/{file_name}.txt");
    let start = time::Instant::now();
    {
        let input = read_to_string(input_file).unwrap();
        let actual = f(&input, params);

        assert_eq!(expected, actual);
    }
    let elapsed = start.elapsed();

    let (time, units) = if elapsed.as_secs() >= 1 {
        (elapsed.as_millis(), "ms")
    } else {
        (elapsed.as_micros(), "μs")
    };

    println!("Elapsed: {time} {units}");
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Num + Copy> Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Num + Copy> AddAssign<Vector2<T>> for Point2<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        *self = *self + rhs;
    }
}

impl<T: Num + Sub<Output = T>> Sub<Vector2<T>> for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Num + Copy> SubAssign<Vector2<T>> for Point2<T> {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        *self = *self - rhs;
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vector2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Copy + Neg<Output = T>> Vector2<T> {
    pub fn rotate_90(&mut self, rotation_direction: RotationDirection) {
        match rotation_direction {
            RotationDirection::Clockwise => {
                let x_prev = self.x;
                self.x = self.y;
                self.y = -x_prev;
            }
            RotationDirection::Counterclockwise => {
                let x_prev = self.x;
                self.x = -self.y;
                self.y = x_prev;
            }
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vector2<T> {
    type Output = Self;
    fn mul(self, num: T) -> Self::Output {
        Self {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

pub enum RotationDirection {
    Clockwise,
    Counterclockwise,
}
