use lazy_static::lazy_static;
use std::{cmp::Eq, env, fmt::Debug, fs::read_to_string, path::PathBuf, time};

lazy_static! {
    pub(crate) static ref MANIFEST_DIR: String = env::var("CARGO_MANIFEST_DIR").unwrap();
}

#[allow(unused)]
pub(crate) fn test<T: Debug + Eq, F: Fn(&str) -> T>(input_file: &PathBuf, f: F, expected: T) {
    test_with_params(input_file, |s: &str, _: ()| f(s), (), expected);
}

#[allow(unused)]
pub(crate) fn test_with_params<P, T: Debug + Eq, F: Fn(&str, P) -> T>(
    input_file: &PathBuf,
    f: F,
    params: P,
    expected: T,
) {
    let start = time::Instant::now();
    {
        let input = read_to_string(input_file).unwrap();
        let actual = f(&input, params);

        assert_eq!(expected, actual);
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {} millis", elapsed.as_millis());
}
