use std::str::FromStr;

use regex::Regex;

// 260,605 -> 260,124

pub fn parse_lines<Out, T>(board_raw: T) -> Vec<Out>
where
    T: Iterator<Item = String>,
    Out: FromStr,
    <Out as FromStr>::Err: std::fmt::Debug,
{
    let re = Regex::new(r",| -> ").unwrap();

    board_raw
        .flat_map(|s| {
            re.split(&s)
                .map(|x| x.parse().unwrap())
                .collect::<Vec<Out>>()
        })
        .collect()
}
