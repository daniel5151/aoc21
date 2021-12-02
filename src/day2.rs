use crate::prelude::*;

enum Cmd {
    Fwd,
    Up,
    Down,
}

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|l| -> DynResult<_> {
                let (cmd, n) = l.split_once(' ').ok_or("missing magnitude")?;
                let cmd = match cmd {
                    "forward" => Cmd::Fwd,
                    "down" => Cmd::Down,
                    "up" => Cmd::Up,
                    _ => return Err("invalid direction".into()),
                };
                let n = n.parse::<isize>()?;
                Ok((cmd, n))
            })
            .collect::<DynResult<Vec<_>>>()?
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<isize> {
    let input = munge_input!(input);

    let mut depth = 0;
    let mut hpos = 0;

    for (cmd, n) in input {
        match cmd {
            Cmd::Fwd => hpos += n,
            Cmd::Up => depth -= n,
            Cmd::Down => depth += n,
        }
    }

    let ans = hpos * depth;

    Ok(ans)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<isize> {
    let input = munge_input!(input);

    let mut depth = 0;
    let mut hpos = 0;
    let mut aim = 0;

    for (cmd, n) in input {
        match cmd {
            Cmd::Fwd => {
                hpos += n;
                depth += aim * n;
            }
            Cmd::Up => aim -= n,
            Cmd::Down => aim += n,
        }
    }

    let ans = hpos * depth;

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 150 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 900 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
