struct BitSummary {
    pub sample_size: usize,
    pub break_down: Vec<u32>,
}

fn get_input() -> impl Iterator<Item = u16> {
    aoc_common::read_lines("resources/aoc-03.input")
        .unwrap()
        .into_iter()
        .map(|line| u16::from_str_radix(&line.unwrap(), 2).unwrap())
}

fn bit_count<T>(list: T, bit_width: usize) -> BitSummary
where
    T: IntoIterator<Item = u16>,
{
    let selector_mask = 1 << (bit_width - 1);
    let summary = BitSummary {
        sample_size: 0,
        break_down: vec![0u32; bit_width],
    };

    list.into_iter().fold(summary, |mut acc, line| {
        acc.break_down
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = *v + ((line & (selector_mask >> i)) > 0) as u32);
        acc.sample_size += 1;
        acc
    })
}

fn part_1<T>(input: T, bit_width: usize) -> u32
where
    T: IntoIterator<Item = u16>,
{
    let padding_mask = !(u32::MAX << bit_width);
    let bit_summary = bit_count(input, bit_width);

    let half_len: u32 = (bit_summary.sample_size / 2).try_into().unwrap();

    let val = bit_summary
        .break_down
        .iter()
        .map(|&v| v > half_len)
        .fold(0, |acc, val| (acc << 1) + val as u32);

    let gamma = val;
    let eps = !val & padding_mask;

    return gamma * eps;
}

fn part_2<T>(input: T, bit_width: usize) -> u64
where
    T: IntoIterator<Item = u16>,
{
    let input_list = input.into_iter().collect::<Vec<_>>();
    let mut _input = input_list.clone();

    for i in 0..bit_width {
        let selector_mask = 1 << (bit_width - 1 - i);

        let count = _input.iter().filter(|&x| (x & selector_mask) != 0).count();
        let logic = selector_mask * (count >= _input.len() - count) as u16;

        _input = _input
            .iter()
            .filter(|&&x| (x & selector_mask) == logic)
            .map(|&x| x)
            .collect::<Vec<_>>();

    	if _input.len() <= 1 {
    		break;
    	}
    }

    let oxy = _input[0];

	let mut _input = input_list;
	for i in 0..bit_width {
        let selector_mask = 1 << (bit_width - 1 - i);

        let count = _input.iter().filter(|&x| (x & selector_mask) != 0).count();

        let logic = selector_mask * (count < _input.len() - count) as u16;

        _input = _input
            .iter()
            .filter(|&&x| (x & selector_mask) == logic)
            .map(|&x| x)
            .collect::<Vec<_>>();

    	if _input.len() <= 1 {
    		break;
    	}
    }
    let co2 = _input[0];

    oxy as u64 * co2 as u64
}

fn main() {
    let part_1_solution = part_1(get_input(), 12);
    println!("Part 1: {}", part_1_solution);

	let part_2_solution = part_2(get_input(), 12);
    println!("Part 2: {}", part_2_solution);
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_1_basic() {
        let input = vec![0b10110, 0b10110];

        let actual = super::part_1(input, 5);
        let expected = 198;

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_example() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let actual = super::part_1(input, 5);
        let expected = 198;

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_basic_1() {
        let ox = 0b11111;
        let co2 = 0b00011;
        let expected = (ox * co2) as u64;

        let input = vec![ox, co2, 0b11110];
        let actual = super::part_2(input, 5);

        assert_eq!(actual, expected);
    }

	#[test]
    fn part_2_basic_2() {
        let ox = 0b00011;
        let co2 = 0b11111;
        let expected = (ox * co2) as u64;

        let input = vec![ox, co2, 0b00001];
        let actual = super::part_2(input, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_2_example() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let actual = super::part_2(input, 5);
        let expected = 230;

        assert_eq!(actual, expected);
    }
}
