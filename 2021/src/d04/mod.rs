use itertools::Itertools;

#[derive(Clone)]
struct BingoBoard {
    board: [[Option<u32>; 5]; 5],
    has_bingo: bool,
}

impl BingoBoard {
    fn parse(input: &[&str]) -> Self {
        let board_vec = input
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| Some(s.trim().parse::<u32>().unwrap()))
                    .collect_vec()
            })
            .collect_vec();

        let mut board = [[Some(0_u32); 5]; 5];
        for (idx, row) in board_vec.iter().enumerate() {
            board[idx].copy_from_slice(row);
        }
        Self {
            board,
            has_bingo: false,
        }
    }

    fn mark(&mut self, bingo_num: u32) {
        let maybe_mark = self.matched_index(bingo_num);
        if let Some((row, col)) = maybe_mark {
            self.board[row][col].take();
            if self.has_bingo {
                return;
            }
            if self.board[row].iter().all(|n| n.is_none()) {
                self.has_bingo = true;
                return;
            }
            if self.board.iter().map(|row| row[col]).all(|n| n.is_none()) {
                self.has_bingo = true;
            }
        }
    }

    fn matched_index(&self, bingo_num: u32) -> Option<(usize, usize)> {
        for (idx_r, row) in self.board.iter().enumerate() {
            for (idx_c, num) in row.iter().enumerate() {
                if num == &Some(bingo_num) {
                    return Some((idx_r, idx_c));
                }
            }
        }
        None
    }

    fn unmarked_nums(&self) -> Vec<u32> {
        self.board.into_iter().flatten().flatten().collect_vec()
    }
}

pub fn solve(problem: &[&str]) -> (u32, u32) {
    let bingo_numbers = problem[0]
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect_vec();
    let bingo_boards = problem[1..]
        .chunks(6)
        .map(|chunk| BingoBoard::parse(&chunk[1..]))
        .collect_vec();
    (solve1(&bingo_numbers, &mut bingo_boards.clone()), solve2(&bingo_numbers, &mut bingo_boards.clone()))
}

fn solve1(bingo_numbers: &[u32], bingo_boards: &mut [BingoBoard]) -> u32 {
    let (winning_board, last_bingo) = winning_board(bingo_numbers, bingo_boards.to_vec()).unwrap();
    let winning_sum: u32 = winning_board.unmarked_nums().iter().sum();
    last_bingo * winning_sum
}

fn solve2(bingo_numbers: &[u32], bingo_boards: &mut [BingoBoard]) -> u32 {
    let (losing_board, last_bingo) = losing_board(bingo_numbers, bingo_boards.to_vec()).unwrap();
    let losing_sum: u32 = losing_board.unmarked_nums().iter().sum();
    last_bingo * losing_sum
}

fn winning_board(
    bingo_numbers: &[u32],
    mut bingo_boards: Vec<BingoBoard>,
) -> Option<(BingoBoard, u32)> {
    for num in bingo_numbers {
        for board in bingo_boards.iter_mut() {
            board.mark(*num);
            if board.has_bingo {
                return Some((board.clone(), *num));
            }
        }
    }
    None
}

fn losing_board(
    bingo_numbers: &[u32],
    mut bingo_boards: Vec<BingoBoard>,
) -> Option<(BingoBoard, u32)> {
    for num in bingo_numbers {
        for board in bingo_boards.iter_mut() {
            board.mark(*num);
        }
        if bingo_boards.len() > 1 {
            bingo_boards.retain(|board| !board.has_bingo);
        } else if bingo_boards[0].has_bingo {
            return Some((bingo_boards[0].clone(), *num));
        }
    }
    None
}


#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn board_parse() {
    let board = r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"#
        .lines()
        .collect_vec();
    let bingo_board = BingoBoard::parse(&board);
    assert_eq!(
        [
            [Some(22), 13.into(), 17.into(), 11.into(), 0.into()],
            [8.into(), 2.into(), 23.into(), 4.into(), 24.into()],
            [21.into(), 9.into(), 14.into(), 16.into(), 7.into()],
            [6.into(), 10.into(), 3.into(), 18.into(), 5.into()],
            [1.into(), 12.into(), 20.into(), 15.into(), 19.into()]
        ],
        bingo_board.board
    )
}

#[test]
fn has_bingo_horizontal() {
    let board = r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"#
        .lines()
        .collect_vec();
    let mut bingo_board = BingoBoard::parse(&board);
    for num in [22, 13, 17, 11] {
        bingo_board.mark(num);
        assert!(!bingo_board.has_bingo);
    }
    bingo_board.mark(0);
    assert!(bingo_board.has_bingo);
}

#[test]
fn has_bingo_vertical() {
    let board = r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"#
        .lines()
        .collect_vec();
    let mut bingo_board = BingoBoard::parse(&board);
    for num in [13, 2, 9, 10] {
        bingo_board.mark(num);
        assert!(!bingo_board.has_bingo);
    }
    bingo_board.mark(12);
    assert!(bingo_board.has_bingo);
}

#[test]
fn correct_unmarked_items_simple() {
    let board = r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"#
        .lines()
        .collect_vec();
    let mut bingo_board = BingoBoard::parse(&board);
    bingo_board.mark(13);

    let mut expected = vec![17, 11, 0, 23, 4, 24, 14, 16, 7, 3, 18, 5, 20, 15, 19, 2, 9, 10, 12, 22, 21, 6, 1, 8];
    expected.sort();
    let mut actual = bingo_board.unmarked_nums();
    actual.sort();

    assert_eq!(expected, actual);
}

#[test]
fn correct_unmarked_items_complex() {
    let board = r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"#
        .lines()
        .collect_vec();
    let mut bingo_board = BingoBoard::parse(&board);
    for num in [13, 2, 9, 10, 12, 22, 21, 6, 1, 8] {
        bingo_board.mark(num);
    }
    let mut expected = vec![17, 11, 0, 23, 4, 24, 14, 16, 7, 3, 18, 5, 20, 15, 19];
    expected.sort();
    let mut actual = bingo_board.unmarked_nums();
    actual.sort();
    assert_eq!(expected, actual);
}
