use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>()
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let res = input
        .windows(2)
        .map(|c| c[1] - c[0])
        .filter(|x| *x > 0)
        .count();

    Ok(res)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let res = input
        .windows(3)
        .map(|c| c.iter().sum())
        .collect::<Vec<isize>>()
        .windows(2)
        .map(|c| c[1] - c[0])
        .filter(|x| *x > 0)
        .count();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 7 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 5 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
