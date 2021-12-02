type Input = Vec<(String, i32)>;

fn get_input() -> Input {
    aoc_common::read_lines("resources/aoc-02.input")
        .unwrap()
        .into_iter()
        .map(|r| {
            r.unwrap()
                .split(' ')
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .map(|s| (s[0].clone(), s[1].parse::<i32>().unwrap()))
        .collect()
}

fn part_1(input: &Input) -> i32 {
    let val = input.iter()
        .fold((0, 0), |acc, (s, i)| match (s.as_ref(), i) {
            ("forward", x) => (acc.0 + x, acc.1),
            ("down", y) => (acc.0, acc.1 + y),
            ("up", y) => (acc.0, acc.1 - y),
            _ => panic!("Unknown value {:?}", (s, i)),
        });

    val.0 * val.1
}

fn part_2(input: &Input) -> i32 {
    let val = input.iter().fold((0, 0, 0), |(x, y, aim), (s, i)| match (s.as_ref(), i) {
            ("forward", dx) => (x + dx, y + (aim * dx), aim),
            ("down", dy) => (x, y, aim + dy),
            ("up", dy) => (x, y, aim - dy),
            _ => panic!("Unknown value {:?}", (s, i)),
        });

    val.0 * val.1
}

fn main() {
    let input = get_input();

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("Solution 1: {}", part_1); // 1499229
    println!("Solution 2: {}", part_2); // 1340836560

    println!("A: {}", (1638421674.0 - 1638423807.0) / 60.0);
    println!("B: {}", (1638421878.0 - 1638424381.0) / 60.0);
}
