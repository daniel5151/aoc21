use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let input = input.split('\n').collect::<HashSet<_>>();
        if !input.iter().map(|s| s.len()).all_equal() {
            return Err("binary strings must be equal length".into());
        }
        let len = input
            .iter()
            .next()
            .ok_or("must have at least one input string")?
            .len();
        (input, len)
    }};
}

type Answer = usize;

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (input, len) = munge_input!(input);

    let mut gamma: usize = 0;

    for i in 0..len {
        let mut more_zero: isize = 0;
        for s in &input {
            match s.as_bytes()[i] {
                b'0' => more_zero += 1,
                b'1' => more_zero -= 1,
                _ => return Err("unexpected char".into()),
            }
        }

        gamma <<= 1;
        gamma |= if more_zero > 0 { 0 } else { 1 };
    }

    // epsilon is just the binary invert of gamma
    let epsilon = !gamma & (!0 >> (core::mem::size_of::<usize>() * 8 - len));

    let ans = gamma * epsilon;

    Ok(ans)
}

fn filter(mut vals: HashSet<&str>, len: usize, most_common: bool) -> DynResult<usize> {
    let mut final_val = None;

    for i in 0..len {
        let mut more_zero: isize = 0;
        let mut with_zero = HashSet::new();
        let mut with_one = HashSet::new();

        for s in &vals {
            match s.as_bytes()[i] {
                b'0' => {
                    more_zero += 1;
                    with_zero.insert(*s);
                }
                b'1' => {
                    more_zero -= 1;
                    with_one.insert(*s);
                }
                _ => return Err("unexpected char".into()),
            }
        }

        if more_zero > 0 {
            vals = if most_common { with_zero } else { with_one }
        } else {
            vals = if most_common { with_one } else { with_zero }
        }

        if vals.len() == 1 {
            final_val = vals.iter().next().copied();
        }
    }

    Ok(usize::from_str_radix(final_val.unwrap(), 2)?)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (input, len) = munge_input!(input);

    let o2 = filter(input.clone(), len, true)?;
    let co2 = filter(input.clone(), len, false)?;

    let ans = co2 * o2;

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 198 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 230 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
