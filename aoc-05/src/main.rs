mod parser;
use std::{collections::HashMap, iter};

use itertools::Itertools;

use crate::parser::parse_lines;

enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

impl From<&[usize]> for Orientation {
    fn from(line: &[usize]) -> Self {
        match line {
            _ if is_vertical(line) => Self::Vertical,
            _ if is_horizontal(line) => Self::Horizontal,
            _ => Self::Diagonal,
        }
    }
}

fn is_vertical(line: &[usize]) -> bool {
    line[0] == line[2]
}

fn is_horizontal(line: &[usize]) -> bool {
    line[1] == line[3]
}

fn part_1(path: &str) -> usize {
    let re = regex::Regex::new(r",| -> ").unwrap();
    let mut summary = HashMap::<(usize, usize), usize>::new();

    let input_raw = aoc_common::read_lines(path)
        .unwrap()
        .flat_map(|s| {
            re.split(&s.unwrap())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    input_raw
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

fn part_2(path: &str) -> usize {
    let re = regex::Regex::new(r",| -> ").unwrap();
    let mut summary = HashMap::<(usize, usize), usize>::new();

    let input_raw = aoc_common::read_lines(path)
        .unwrap()
        .flat_map(|s| {
            re.split(&s.unwrap())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    input_raw
        .chunks(4)
        .flat_map(|line| {
            let mut line = [(line[0], line[1]), (line[2], line[3])];
            line.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));

            let range_x: Box<dyn Iterator<Item = usize>> = match line[0].0.cmp(&line[1].0) {
                std::cmp::Ordering::Equal => Box::new(iter::repeat(line[0].0)),
                _ => Box::new(line[0].0..=line[1].0),
            };

            let range_y: Box<dyn Iterator<Item = usize>> = match line[0].1.cmp(&line[1].1) {
                std::cmp::Ordering::Less => Box::new(line[0].1..=line[1].1),
                std::cmp::Ordering::Greater => Box::new((line[1].1..=line[0].1).rev()),
                std::cmp::Ordering::Equal => Box::new(iter::repeat(line[0].1)),
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
    println!("{}", part_1("resources/aoc-05.input"));
    println!("{}", part_2("resources/aoc-05.input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let expected: usize = 5;
        let actual = part_1("test.input");

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_test() {
        let expected: usize = 12;
        let actual = part_2("test.input");

        assert_eq!(actual, expected);
    }
}
