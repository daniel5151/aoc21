// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = {
        let mut input = input.split(',');

        // let init = input.next().unwrap();

        let input = input
            .map(|s| -> DynResult<_> {
                let res = { s.parse::<usize>()? };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut fish = input;
    for _ in 0..80 {
        let mut new = Vec::new();
        for t in &mut fish {
            if *t == 0 {
                *t = 6;
                new.push(8)
            } else {
                *t -= 1;
            }
        }
        fish.extend(new);
    }

    Ok(fish.len())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = {
        let mut input = input.split(',');

        // let init = input.next().unwrap();

        let input = input
            .map(|s| -> DynResult<_> {
                let res = { s.parse::<usize>()? };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut schools = [0; 9];

    for f in input {
        schools[f] += 1;
    }

    for _ in 0..256 {
        let new = schools[0];
        schools.rotate_left(1);
        schools[8] = new;
        schools[6] += new;
    }

    Ok(schools.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"3,4,3,1,2"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 5934 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 26984457539 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
