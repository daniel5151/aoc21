use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;

        let mut input = input.split("\n\n");

        let nums = input
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?;

        let boards = input
            .map(|s| -> DynResult<_> {
                let mut b: [[(u8, bool); 5]; 5] = Default::default();
                for (ln, b_row) in s.split('\n').zip(b.iter_mut()) {
                    for (n, b_e) in ln.split_whitespace().zip(b_row.iter_mut()) {
                        *b_e = (n.parse::<u8>()?, false);
                    }
                }

                Ok(Board { board: b })
            })
            .collect::<Result<Vec<_>, _>>()?;

        (nums, boards)
    }};
}

#[derive(Clone, Copy)]
struct Board {
    board: [[(u8, bool); 5]; 5],
}

impl Board {
    fn check(&self) -> bool {
        // horizontal
        if self
            .board
            .into_iter()
            .map(|row| row.into_iter().all(|(_, c)| c))
            .any(|row| row)
        {
            return true;
        }

        // vertical
        for col in 0..5 {
            if self
                .board
                .into_iter()
                .map(|row| row[col].1)
                .fold(true, |a, c| a & c)
            {
                return true;
            }
        }

        false
    }

    fn update(&mut self, n: u8) {
        for (e, c) in self.board.iter_mut().flat_map(|row| row.iter_mut()) {
            if *e == n {
                *c = true;
            }
        }
    }

    fn score(&self) -> usize {
        self.board
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|(_, c)| !c)
            .map(|(n, _)| n as usize)
            .sum()
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.into_iter() {
            writeln!(f)?;
            for (n, c) in row {
                let (l, r) = if c {
                    // black text on green background
                    ("\x1b[30;42m", "\x1b[0m")
                } else {
                    ("", "")
                };
                write!(f, "{}{:2?}{} ", l, n, r)?;
            }
        }
        Ok(())
    }
}

type Answer = usize;

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input!(input);

    let (nums, mut boards) = input;

    for num in nums {
        for board in boards.iter_mut() {
            board.update(num);
            if board.check() {
                dbg!(&board);
                return Ok(num as usize * board.score());
            }
        }
    }

    Err("no winning board".into())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input!(input);

    let (nums, mut boards) = input;

    for num in nums {
        for board in &mut boards {
            board.update(num);
        }

        if boards.len() == 1 && boards[0].check() {
            return Ok(num as usize * boards[0].score());
        }

        boards.retain(|board| !board.check());
    }

    Err("no winning board".into())
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
