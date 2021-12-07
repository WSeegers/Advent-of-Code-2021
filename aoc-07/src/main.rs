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

fn solve<T>(positions: &[i64], fuel_calc : T) -> i64 where T : Fn(i64) -> i64{
    let (min, max) = match positions.iter().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min, max),
        _ => panic!(),
    };

    positions
        .into_iter()
        .flat_map(|pos| (min..max).zip(std::iter::repeat(pos)))
        .map(|(offset, pos)| (pos - offset).abs())
        .chunks((max - min) as usize)
        .into_iter()
        .fold(vec![0; (max - min) as usize], |mut acc, fuel| {
            acc.iter_mut().zip(fuel).for_each(|(a, f)| *a += fuel_calc(f));
            acc
        })
        .into_iter()
        .min()
        .unwrap()
}

fn part_1(n: i64) -> i64 {
    n
}

fn part_2(n : i64) -> i64 {
    (n  * (n + 1)) / 2
}

fn main() {
    let input = load_input("resources/aoc-07.input");

    println!("Part1: {}", solve(&input, part_1));
    println!("Part2: {}", solve(&input, part_2));
}

mod tests {
    use super::*;

    #[test]
    fn load() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let expected = 37;
        let actual = solve(&input, part_1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn p_2() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let expected = 168;
        let actual = solve(&input,part_2);

        assert_eq!(actual, expected);
    }
}
