use itertools::{Itertools, MinMaxResult};
use parser::{parse_bingo_calls, parse_boards};
use std::collections::HashMap;

mod parser;

struct BoardCell {
    value: usize,
    turn_called: usize,
}

struct Evaluation {
    winning_turn: usize,
    remaining_total: usize,
}

fn load_input(path: &str) -> impl Iterator<Item = String> {
    aoc_common::read_lines(path).unwrap().map(|x| x.unwrap())
}

fn evaluate_board(cells: &[BoardCell]) -> Evaluation {
    let col_winning_turns = cells
        .iter()
        .enumerate()
        .fold(vec![0; 5], |mut acc, (i, x)| {
            acc[i % 5] = acc[i % 5].max(x.turn_called);
            acc
        })
        .into_iter();

    let row_winning_turns = cells.chunks(5).map(|x| {
        x.iter()
            .map(|cell| cell.turn_called)
            .max()
            .unwrap()
            .to_owned()
    });

    let winning_turn = col_winning_turns.chain(row_winning_turns).min().unwrap();

    let board_value = cells
        .into_iter()
        .filter_map(
            |&BoardCell { value, turn_called }| match turn_called > winning_turn {
                true => Some(value.to_owned()),
                false => None,
            },
        )
        .sum::<usize>();

    Evaluation {
        winning_turn,
        remaining_total: board_value,
    }
}

fn bingo_min_max(path: &str) -> (usize, usize) {
    let mut input_iter = load_input(path).into_iter();

    let numbers: Vec<usize> = parse_bingo_calls(&input_iter.next().unwrap());
    let numbers_lookup = numbers
        .iter()
        .enumerate()
        .map(|(position, &n)| (n, position))
        .collect::<HashMap<_, _>>();

    let boards_values = input_iter
        .chunks(6)
        .into_iter()
        .flat_map(|x| parse_boards(x.skip(1)))
        .map(|x| BoardCell {
            value: x,
            turn_called: numbers_lookup.get(&x).unwrap_or(&usize::MAX).to_owned(),
        })
        .collect::<Vec<_>>();

    let board_iter = boards_values.chunks(25);

    let evaluation = board_iter.map(evaluate_board);

    let (first, last) = match evaluation.minmax_by(|x, y| x.winning_turn.cmp(&y.winning_turn)) {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!(),
    };

    (
        numbers[first.winning_turn] * first.remaining_total,
        numbers[last.winning_turn] * last.remaining_total,
    )
}

fn main() {
    println!("{:?}", bingo_min_max("resources/aoc-04.input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_basic() {
        let expected: usize = 4512;
        let actual = bingo_min_max("test.input").0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_basic() {
        let expected: usize = 1924;
        let actual = bingo_min_max("test.input").1;

        assert_eq!(actual, expected);
    }
}
