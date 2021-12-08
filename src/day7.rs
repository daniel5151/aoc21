use crate::prelude::*;

type Answer = usize;

type Input = Vec<isize>;
fn munge_input(input: &str) -> DynResult<Input> {
    input
        .split(',')
        .map(|s| s.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn min_fuel_pos(crabs: Vec<isize>, cost_fn: fn(isize, isize) -> isize) -> DynResult<Answer> {
    let (min, max) = crabs
        .iter()
        .copied()
        .minmax()
        .into_option()
        .ok_or("empty input")?;

    let mut min_fuel = isize::MAX;
    for pos in min..=max {
        let mut fuel = 0;
        for crab in crabs.iter().copied() {
            fuel += cost_fn(pos, crab);
        }
        min_fuel = min_fuel.min(fuel);
    }

    Ok(min_fuel as usize)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;
    min_fuel_pos(input, |a, b| (a - b).abs())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;
    min_fuel_pos(input, |a, b| {
        let n = (a - b).abs();
        (n * (n + 1)) / 2
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 37 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 168 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
