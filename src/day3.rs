use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input.split('\n').map(|s| s.as_bytes()).collect::<Vec<_>>()
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut g: usize = 0;

    let len = input[0].len();
    for i in 0..len {
        let mut more_zero: isize = 0;
        for s in &input {
            match s[i] {
                b'0' => more_zero += 1,
                b'1' => more_zero -= 1,
                _ => panic!(),
            }
        }
        g <<= 1;
        g += if more_zero > 0 { 0 } else { 1 };
    }

    let ans = g * (!g & (!0usize >> (64 - len)));

    Ok(ans)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = input.split('\n').collect::<HashSet<_>>();
    let len = input.iter().next().unwrap().len();

    let mut o2 = input.clone();
    let mut co2 = input.clone();

    let mut o2_val = None;
    let mut co2_val = None;

    for i in 0..len {
        {
            let mut more_zero: isize = 0;
            let mut with_zero = HashSet::new();
            let mut with_one = HashSet::new();

            for s in &o2 {
                match s.as_bytes()[i] {
                    b'0' => {
                        more_zero += 1;
                        with_zero.insert(*s);
                    }
                    b'1' => {
                        more_zero -= 1;
                        with_one.insert(*s);
                    }
                    _ => panic!(),
                }
            }

            let next_o2: HashSet<_>;

            if more_zero > 0 {
                next_o2 = with_zero;
            } else {
                next_o2 = with_one;
            }

            o2 = next_o2;

            if o2.len() == 1 {
                o2_val = o2.iter().next().copied();
            }
        }

        {
            let mut more_zero: isize = 0;
            let mut with_zero = HashSet::new();
            let mut with_one = HashSet::new();

            for s in &co2 {
                match s.as_bytes()[i] {
                    b'0' => {
                        more_zero += 1;
                        with_zero.insert(*s);
                    }
                    b'1' => {
                        more_zero -= 1;
                        with_one.insert(*s);
                    }
                    _ => panic!(),
                }
            }

            let next_co2: HashSet<_>;

            if more_zero > 0 {
                next_co2 = with_one;
            } else {
                next_co2 = with_zero;
            }

            co2 = next_co2;

            if co2.len() == 1 {
                co2_val = co2.iter().next().copied();
            }
        }
    }

    let o2 = usize::from_str_radix(o2_val.unwrap(), 2).unwrap();
    let co2 = usize::from_str_radix(co2_val.unwrap(), 2).unwrap();

    let ans = o2 * co2;

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
