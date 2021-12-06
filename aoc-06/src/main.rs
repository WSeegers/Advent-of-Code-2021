use std::path::Path;

type School = [usize; 9];

fn load_input<P>(path: P) -> School
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
        .fold([0; 9], |mut acc, c: usize| {
            acc[c] += 1;
            acc
        })
}

fn progress_day(mut school: School, d: usize) -> School {
    let spawn = school[d % 7];
    school[d % 7] += school[7];
    school[7] = school[8];
    school[8] = spawn;
    school
}

fn execute(v: School, days: usize) -> usize {
    (0..days)
        .fold(v, |acc, day| progress_day(acc, day))
        .iter()
        .sum()
}

fn main() {
    let part1 = execute(load_input("resources/aoc-06.input"), 80);
    println!("Part1: {}", part1);

    let part2 = execute(load_input("resources/aoc-06.input"), 256);
    println!("Part2: {}", part2);
}

mod tests {
    use super::*;

    #[test]
    fn load() {
        let expected =  [0, 1, 1, 2, 1, 0, 0, 0, 0];
        let actual = load_input("test.input");

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_1_day() {
        let expected = [0, 1, 1, 2, 1, 0, 0, 0, 0];
        let actual = progress_day(load_input("test.input"), 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_2_days() {
        let expected = 6;
        let actual = execute(load_input("test.input"), 2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_3_days() {
        let expected = 7;
        let actual = execute(load_input("test.input"), 3);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_18_days() {
        let expected = 26;
        let actual = execute(load_input("test.input"), 18);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_80_days() {
        let expected = 5934;
        let actual = execute(load_input("test.input"), 80);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_256_days() {
        let expected = 26984457539;
        let actual = execute(load_input("test.input"), 256);

        assert_eq!(actual, expected);
    }
}
