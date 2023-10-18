use itertools::Itertools;

pub fn parse_example_testcases(input: &Vec<String>) -> Vec<TestCase> {
    // find indeces of test case boundaries
    let indeces_iter = input
        .iter()
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
    fn parse_u32_from_string(input: &String) -> u32 {
        let maybe_num = input.split(':').last().unwrap().trim();

        if maybe_num == "-" {
            return 0;
        }
        maybe_num.parse().unwrap()
    }

    indeces_iter
        .chunks(3)
        .map(|indeces| {
            let (start, middle) = (indeces[0], indeces[1]);
            let problem = input
                .get(start + 1..middle)
                .unwrap()
                .iter()
                .map(String::from)
                .collect_vec();

            let answer1: u32 = input.get(middle + 1).map(parse_u32_from_string).unwrap();
            let answer2: u32 = input.get(middle + 2).map(parse_u32_from_string).unwrap();

            TestCase {
                problem,
                answer1,
                answer2,
            }
        })
        .collect_vec()
}

#[derive(Debug, PartialEq)]
pub struct TestCase {
    pub problem: Vec<String>,
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
"#
    .lines()
    .map(String::from)
    .collect_vec();
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
"#
    .lines()
    .map(String::from)
    .collect_vec();
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
