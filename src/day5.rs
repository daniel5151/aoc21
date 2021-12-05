use crate::prelude::*;

type Answer = usize;

// using isize as input vals never exceed 1000
type Input = Vec<((isize, isize), (isize, isize))>;

fn munge_input(input: &str) -> DynResult<Input> {
    input
        .split('\n')
        .map(|s| -> DynResult<_> {
            let res = {
                // parse
                let (n1, n2) = s.split_once(" -> ").ok_or("misisng ->")?;
                let (x1, y1) = n1.split_once(',').ok_or("missing first , separator")?;
                let (x2, y2) = n2.split_once(',').ok_or("missing second , separator")?;
                (
                    (x1.parse::<isize>()?, y1.parse::<isize>()?),
                    (x2.parse::<isize>()?, y2.parse::<isize>()?),
                )
            };
            Ok(res)
        })
        .collect::<Result<Vec<_>, _>>()
}

fn tally(input: Input, count_diag: bool) -> DynResult<Answer> {
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
            if !count_diag {
                continue;
            }

            let ((sx, sy), (ex, ey)) = {
                if x1 < x2 {
                    ((x1, y1), (x2, y2))
                } else {
                    ((x2, y2), (x1, y1))
                }
            };

            let dy = if sy < ey { 1 } else { -1 };

            let mut y = sy;
            for x in sx..=ex {
                *points.entry((x, y)).or_default() += 1;
                y += dy;
            }
        }
    }

    let ans = points.into_values().filter(|c| *c > 1).count();

    Ok(ans)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;
    tally(input, false)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;
    tally(input, true)
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
