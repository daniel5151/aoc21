use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|l| {
                let mut s = l.split(' ');
                let dir = s.next().unwrap();
                let n = s.next().unwrap().parse::<isize>().unwrap();
                (dir, n)
            })
            .collect::<Vec<_>>()
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<isize> {
    let input = munge_input!(input);

    let mut a = 0;
    let mut b = 0;
    for (dir, n) in input {
        match dir {
            "forward" => a += n,
            "down" => b += n,
            "up" => b -= n,
            _ => panic!(),
        }
    }

    Ok(a * b)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<isize> {
    let input = munge_input!(input);

    let mut depth = 0;
    let mut hpos = 0;
    let mut aim = 0;
    for (dir, n) in input {
        match dir {
            "forward" => {
                hpos += n;
                depth += aim * n;
            }

            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!(),
        }
    }

    Ok(hpos * depth)
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
