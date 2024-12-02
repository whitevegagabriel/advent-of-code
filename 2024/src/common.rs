use std::{cmp::Eq, env, fmt::Debug, fs::read_to_string, thread::sleep, time::{self, Duration}};

#[allow(unused)]
pub(crate) fn test<T: Debug + Eq, F: Fn(&str) -> T>(
    input_type: &PuzzleInputType,
    module_path: &str,
    f: F,
    expected: T,
) {
    test_with_params(input_type, module_path, |s: &str, _: ()| f(s), (), expected);
}

#[allow(unused)]
pub(crate) fn test_with_params<P, T: Debug + Eq, F: Fn(&str, P) -> T>(
    input_type: &PuzzleInputType,
    module_path: &str,
    f: F,
    params: P,
    expected: T,
) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let module_name = module_path.split("::").last().unwrap();
    let file_name = match input_type {
        PuzzleInputType::Input => "input",
        PuzzleInputType::Example => "example",
    };
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

#[allow(unused)]
pub(crate) enum PuzzleInputType {
    Input,
    Example,
}
