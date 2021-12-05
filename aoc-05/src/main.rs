use std::{collections::HashMap, iter, path::Path};

use itertools::Itertools;

enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

impl From<&[usize]> for Orientation {
    fn from(line: &[usize]) -> Self {
        match line {
            _ if line[0] == line[2] => Self::Vertical,
            _ if line[1] == line[3] => Self::Horizontal,
            _ => Self::Diagonal,
        }
    }
}

fn load_input<P>(path: P) -> Vec<usize> where P: AsRef<Path>{
    let re = regex::Regex::new(r",| -> ").unwrap();
    aoc_common::read_lines(path)
        .unwrap()
        .flat_map(|s| {
            re.split(&s.unwrap())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn part_1(input: &[usize]) -> usize {
    let mut summary = HashMap::<(usize, usize), usize>::new();

    input
        .chunks(4)
        .flat_map(|line| match Orientation::from(line) {
            Orientation::Vertical => {
                let mut range = [line[1], line[3]];
                range.sort();
                iter::repeat(line[0]).zip(range[0]..=range[1]).collect()
            }
            Orientation::Horizontal => {
                let mut range = [line[0], line[2]];
                range.sort();
                (range[0]..=range[1]).zip(iter::repeat(line[1])).collect()
            }
            _ => Vec::<(usize, usize)>::new(),
        })
        .for_each(|x| {
            let e = summary.entry(x).or_insert(0);
            *e += 1;
        });

    summary
        .iter()
        .map(|(_, &count)| count)
        .filter(|&count| count > 1)
        .count()
}

fn part_2(input: &[usize]) -> usize {
    let mut summary = HashMap::<(usize, usize), usize>::new();

    input
        .iter()
        .tuples()
        .flat_map(|(&x1, &y1, &x2, &y2)| {
            let mut line = [(x1, y1), (x2, y2)];
            line.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));

            let [(x1, y1), (x2, y2)] = line;

            let range_x: Box<dyn Iterator<Item = usize>> = match x1.cmp(&x2) {
                std::cmp::Ordering::Equal => Box::new(iter::repeat(x1)),
                _ => Box::new(x1..=x2),
            };

            let range_y: Box<dyn Iterator<Item = usize>> = match y1.cmp(&y2) {
                std::cmp::Ordering::Less => Box::new(y1..=y2),
                std::cmp::Ordering::Greater => Box::new((y2..=y1).rev()),
                std::cmp::Ordering::Equal => Box::new(iter::repeat(y1)),
            };

            (range_x).zip(range_y).collect::<Vec<_>>()
        })
        .for_each(|x| {
            let e = summary.entry(x).or_insert(0);
            *e += 1;
        });

    summary
        .iter()
        .map(|(_, &count)| count)
        .filter(|&count| count > 1)
        .count()
}

fn main() {
    let input = load_input("resources/aoc-05.input");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let expected: usize = 5;
        let actual = part_1(&load_input("test.input"));

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_test() {
        let expected: usize = 12;
        let actual = part_2(&load_input("test.input"));

        assert_eq!(actual, expected);
    }
}
