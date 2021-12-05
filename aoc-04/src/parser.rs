use std::str::FromStr;

pub fn parse_bingo_calls<O>(number_line: &String) -> Vec<O>
where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    number_line.split(",").map(|n| n.parse().unwrap()).collect()
}

pub fn parse_boards<O, T>(board_raw: T) -> Vec<O>
where
    T: Iterator<Item = String>,
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
{
    board_raw
        .flat_map(|s| {
            s.split(" ")
                .filter(|&s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
