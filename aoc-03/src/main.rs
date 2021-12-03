use itertools::Itertools;

fn get_input() -> impl Iterator<Item = u16> {
    aoc_common::read_lines("resources/aoc-03.input")
        .unwrap()
        .into_iter()
        .map(|line| u16::from_str_radix(&line.unwrap(), 2).unwrap())
}

fn part_1<T>(input: T, bit_width: usize) -> u64
where
    T: IntoIterator<Item = u16>,
{
	let selector_mask = 1 << (bit_width - 1);

    let  (len, groups) = input
        .into_iter()
        .fold((0, vec![0u32; bit_width]), |(len, mut groups), line| {
            groups
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = *v + ((line & (selector_mask >> i)) > 0) as u32);
            (len + 1, groups)
        });

    let val = groups
        .into_iter()
        .map(|v| (v > len - v) as u64)
        .fold(0, |acc, val| (acc << 1) + val);

    let gamma = val;
    let eps = !val & !(u64::MAX << bit_width);

	return gamma * eps;
}

fn part_2<T>(input: T, bit_width: usize) -> u64
where
    T: IntoIterator<Item = u16>,
{
    let input_list = input.into_iter().collect::<Vec<_>>();
    let mut working_list = input_list.clone();

    for i in 0..bit_width {
        let selector_mask = 1 << (bit_width - 1 - i);

        working_list = working_list
            .into_iter()
            .into_group_map_by(|&x| ((x & selector_mask) != 0) as u8)
            .into_iter()
            .max_by(|(b1, v1), (b2, v2)| {
                let len_cmp = v1.len().cmp(&v2.len());
                match len_cmp {
                    std::cmp::Ordering::Equal => b1.cmp(b2),
                    _ => len_cmp,
                }
            })
            .unwrap()
            .1;

        if working_list.len() <= 1 {
            break;
        }
    }

    let oxy = working_list[0];

    let mut working_list = input_list;
    for i in 0..bit_width {
        let selector_mask = 1 << (bit_width - 1 - i);

        working_list = working_list
            .into_iter()
            .into_group_map_by(|&x| ((x & selector_mask) != 0) as u8)
            .into_iter()
            .min_by(|(b1, v1), (b2, v2)| {
                let len_cmp = v1.len().cmp(&v2.len());
                match len_cmp {
                    std::cmp::Ordering::Equal => b1.cmp(b2),
                    _ => len_cmp,
                }
            })
            .unwrap()
            .1;

        if working_list.len() <= 1 {
            break;
        }
    }
    let co2 = working_list[0];

    oxy as u64 * co2 as u64
}

fn main() {
    let part_1_solution = part_1(get_input(), 12);
    println!("Part 1: {}", part_1_solution); // 3687446

    let part_2_solution = part_2(get_input(), 12);
    println!("Part 2: {}", part_2_solution); // 4406844
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
