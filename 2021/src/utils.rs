use itertools::Itertools;

pub type SolverFn = fn(&[&str]) -> (u32, u32);

#[allow(dead_code)]
pub fn basic_test(input: &str, test: SolverFn) {
    let examples = parse_example_testcases(input);
    for (idx, example) in examples.iter().enumerate() {
        println!("Example {}", idx + 1);
        let (answer1, answer2) = test(&example.problem);
        assert_eq!(example.answer1, answer1);
        assert_eq!(example.answer2, answer2);
    }
}

pub fn parse_example_testcases(input: &str) -> Vec<TestCase> {
    // find indices of test case boundaries
    let indices_vec = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.starts_with("-------------------------------") {
                Some(idx)
            } else {
                None
            }
        })
        .collect_vec();

    // assumes the format "answer_x: i"
    fn parse_u32_from_string(input: &str) -> u32 {
        let maybe_num = input.split(':').last().unwrap().trim();

        if maybe_num == "-" {
            return 0;
        }
        maybe_num.parse().unwrap()
    }

    indices_vec
        .chunks(3)
        .map(|indices| {
            let (start, middle) = (indices[0], indices[1]);
            let problem = input
                .lines()
                .skip(start + 1)
                .take(middle - start - 1)
                .collect_vec();

            let answer1: u32 = input
                .lines()
                .nth(middle + 1)
                .map(parse_u32_from_string)
                .unwrap();
            let answer2: u32 = input
                .lines()
                .nth(middle + 2)
                .map(parse_u32_from_string)
                .unwrap();

            TestCase {
                problem,
                answer1,
                answer2,
            }
        })
        .collect_vec()
}

#[derive(Debug, PartialEq)]
pub struct TestCase<'a> {
    pub problem: Vec<&'a str>,
    pub answer1: u32,
    pub answer2: u32,
}

#[test]
fn parses_single_test_case() {
    let input = r#"
some text
-------------------------------
1
2
3
-------------------------------
hello: 1
world: 2
-------------------------------
"#;
    let test_cases = parse_example_testcases(&input);

    assert_eq!(
        vec![TestCase {
            problem: vec!["1".into(), "2".into(), "3".into()],
            answer1: 1,
            answer2: 2,
        }],
        test_cases
    );
}

#[test]
fn parses_two_test_cases() {
    let input = r#"
some text
-------------------------------
1
2
3
-------------------------------
hello: 1
world: 2
-------------------------------
more text

-------------------------------
greetings...
yo
sup
-------------------------------
hello: -
world: 100
-------------------------------
"#;
    let test_cases = parse_example_testcases(&input);

    assert_eq!(
        vec![
            TestCase {
                problem: vec!["1".into(), "2".into(), "3".into()],
                answer1: 1,
                answer2: 2,
            },
            TestCase {
                problem: vec!["greetings...".into(), "yo".into(), "sup".into()],
                answer1: 0,
                answer2: 100,
            }
        ],
        test_cases
    );
}
