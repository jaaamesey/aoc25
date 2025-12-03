pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let output = input_str
        .lines()
        .map(|line| {
            let line_as_numbers = line
                .chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let mut largest_from_left_idx = 0;
            for (i, c) in line_as_numbers.iter().enumerate() {
                if c > &line_as_numbers[largest_from_left_idx] && i != line_as_numbers.len() - 1 {
                    largest_from_left_idx = i;
                }
            }
            let mut largest_from_right_idx = line_as_numbers.len() - 1;
            for (i, c) in line_as_numbers.iter().enumerate().rev() {
                if i == largest_from_left_idx {
                    break;
                }
                if c > &line_as_numbers[largest_from_right_idx] {
                    largest_from_right_idx = i;
                }
            }
            (line_as_numbers[largest_from_left_idx] * 10) + line_as_numbers[largest_from_right_idx]
        })
        .sum::<u32>();
    dbg!(output);
}

pub fn part2() {
    let input_str = include_str!("./test_input.txt");
}
