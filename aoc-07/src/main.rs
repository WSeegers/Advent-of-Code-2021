use std::path::Path;

use itertools::{Itertools, MinMaxResult};

fn load_input<P>(path: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    aoc_common::read_lines(path)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve<T>(positions: &[i64], fuel_calc: T) -> i64
where
    T: Fn(i64) -> i64,
{
    let (min, max) = match positions.iter().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min, max),
        _ => panic!(),
    };

    (min..max).map(|destination| {
        positions
            .iter()
            .map(|&pos| fuel_calc((destination - pos).abs()))
            .sum::<i64>()
    }).min().unwrap()
}

// Part 1
fn linear_fuel_calc(n: i64) -> i64 {
    n
}

// Part 2
fn exponential_fuel_calc(n: i64) -> i64 {
    (n * (n + 1)) / 2
}

fn main() {
    let input = load_input("resources/aoc-07.input");

    println!("Part1: {}", solve(&input, linear_fuel_calc));
    println!("Part2: {}", solve(&input, exponential_fuel_calc));
}

mod tests {
    use super::*;

    #[test]
    fn load() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let expected = 37;
        let actual = solve(&input, linear_fuel_calc);

        assert_eq!(actual, expected);
    }

    #[test]
    fn p_2() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let expected = 168;
        let actual = solve(&input, exponential_fuel_calc);

        assert_eq!(actual, expected);
    }
}
