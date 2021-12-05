// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = {
        let mut input = input.split('\n');

        // let init = input.next().unwrap();

        let input = input
            .map(|s| -> DynResult<_> {
                let res = {
                    // parse
                    let (n1, n2) = s.split_once(" -> ").ok_or("misisng ->")?;
                    let (x1, y1) = n1.split_once(',').ok_or("missing , separator")?;
                    let (x2, y2) = n2.split_once(',').ok_or("missing , separator")?;

                    (
                        (x1.parse::<isize>()?, y1.parse::<isize>()?),
                        (x2.parse::<isize>()?, y2.parse::<isize>()?),
                    )
                };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut points: HashMap<(isize, isize), isize> = Default::default();

    for ((x1, y1), (x2, y2)) in input {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                *points.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                *points.entry((x, y1)).or_default() += 1;
            }
        } else {
            // skip in q1
            continue;
        }
    }

    let ans = points.into_iter().filter(|(_, count)| *count > 1).count();

    Ok(ans)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = {
        let mut input = input.split('\n');

        // let init = input.next().unwrap();

        let input = input
            .map(|s| -> DynResult<_> {
                let res = {
                    // parse
                    let (n1, n2) = s.split_once(" -> ").ok_or("misisng ->")?;
                    let (x1, y1) = n1.split_once(',').ok_or("missing , separator")?;
                    let (x2, y2) = n2.split_once(',').ok_or("missing , separator")?;

                    (
                        (x1.parse::<isize>()?, y1.parse::<isize>()?),
                        (x2.parse::<isize>()?, y2.parse::<isize>()?),
                    )
                };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut points: HashMap<(isize, isize), isize> = Default::default();

    for ((x1, y1), (x2, y2)) in input {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                *points.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                *points.entry((x, y1)).or_default() += 1;
            }
        } else {
            let start_x;
            let start_y;
            let end_x;
            let y_update;

            if x1 < x2 {
                start_x = x1;
                start_y = y1;
                end_x = x2;

                if y1 < y2 {
                    y_update = 1;
                } else {
                    y_update = -1;
                }
            } else {
                start_x = x2;
                start_y = y2;
                end_x = x1;

                if y2 < y1 {
                    y_update = 1;
                } else {
                    y_update = -1;
                }
            }

            let mut y = start_y;
            for x in start_x..=end_x {
                *points.entry((x, y)).or_default() += 1;
                y += y_update;
            }
        }
    }

    let ans = points.into_iter().filter(|(_, count)| *count > 1).count();

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 5 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 12 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
