use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;

        let mut input = input.split('\n');

        let nums = input
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        input.next().unwrap();

        let mut boards: Vec<[[(usize, bool); 5]; 5]> = Vec::new();

        let mut board = [[(0, false); 5]; 5];
        let mut row_i = 0;

        for ln in input {
            if ln.is_empty() {
                boards.push(board);
                board = [[(0, false); 5]; 5];
                row_i = 0;
                continue;
            }

            let row = ln
                .split_whitespace()
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;

            for (i, e) in row.into_iter().enumerate() {
                board[row_i][i] = (e, false);
            }

            row_i += 1;
        }
        boards.push(board);

        (nums, boards)
    }};
}

type Answer = usize;

fn check_board(board: [[(usize, bool); 5]; 5]) -> bool {
    // horiz
    if board
        .into_iter()
        .map(|row| row.into_iter().all(|(_, c)| c))
        .any(|s| s)
    {
        return true;
    }

    for col in 0..5 {
        let mut n = 0;
        for row in board {
            n += if row[col].1 { 1 } else { 0 };
        }
        if n == 5 {
            return true;
        }
    }

    // if board[0][0].1 && board[1][1].1 && board[2][2].1 && board[3][3].1 &&
    // board[4][4].1 {     return true;
    // }

    // if board[0][4].1 && board[1][3].1 && board[2][2].1 && board[3][1].1 &&
    // board[4][0].1 {     return true;
    // }

    false
}

fn update_board(board: &mut [[(usize, bool); 5]; 5], n: usize) {
    for row in board {
        for e in row {
            if e.0 == n {
                e.1 = true;
            }
        }
    }
}

fn uncalled_board(board: [[(usize, bool); 5]; 5]) -> usize {
    let mut sum = 0;
    for row in board {
        for (n, checked) in row {
            if !checked {
                sum += n;
            }
        }
    }
    sum
}

fn print_board(board: [[(usize, bool); 5]; 5]) -> String {
    use std::fmt::Write;

    let mut s = String::new();
    for row in board {
        writeln!(s).unwrap();
        for (n, i) in row {
            write!(s, "{:2?} {} ", n, if i { "X" } else { " " }).unwrap();
        }
    }
    s
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (nums, mut boards) = munge_input!(input);

    for num in nums {
        for board in &mut boards {
            update_board(board, num);
            if check_board(*board) {
                return Ok(num * uncalled_board(*board));
            }
        }
    }

    unreachable!()
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (nums, mut boards) = munge_input!(input);

    for num in nums {
        if boards.len() == 1 {
            update_board(&mut boards[0], num);
            if check_board(boards[0]) {
                return Ok(num * uncalled_board(boards[0]));
            }
        }
        for board in &mut boards {
            update_board(board, num);
        }
        boards.retain(|board| !check_board(*board));
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 4512 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 1924 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
