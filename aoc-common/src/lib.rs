use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_input<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    read_lines(filename)?.collect()
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_csv<P, F, Out>(path: P, parse: F) -> Vec<Out>
where
    P: AsRef<Path>,
    F: FnMut(&str) -> Out
{
    read_lines(path)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(parse)
        .collect()
}
