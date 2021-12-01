fn count_ascending_neighbours(values: &[i32], window_size: usize) -> usize {
    values
		.windows(window_size)
		.map(|v| v.iter().sum())
		.collect::<Vec<i32>>()
		.windows(2)
		.filter(|&v| v[0] < v[1])
		.count()
}

fn load_values() -> Vec<i32> {
	aoc_common::read_lines("resources/aoc-01.input")
        .unwrap()
        .into_iter()
        .map(|s| s.unwrap().parse().unwrap())
        .collect::<Vec<i32>>()
}

fn main() {
    let values = load_values();

    println!("Part1 Count: {}", count_ascending_neighbours(&values, 1));
    println!("Part2 Count: {}", count_ascending_neighbours(&values, 3));
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_ascending() {
        let count = super::count_ascending_neighbours(&[0, 1, 2], 1);

        assert_eq!(count, 2);
    }

    #[test]
    fn mixed_basic() {
        let count = super::count_ascending_neighbours(&[0, 2, 1], 1);

        assert_eq!(count, 1);
    }

    #[test]
    fn aoc_example_1() {
        let vals = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = super::count_ascending_neighbours(vals, 1);

        assert_eq!(count, 7);
    }

	#[test]
    fn aoc_example_2() {
        let vals = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = super::count_ascending_neighbours(vals, 3);

        assert_eq!(count, 5);
    }
}
