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
    let num_digits: usize = 12;
    let input_str = include_str!("./real_input.txt");
    let output = input_str
        .lines()
        .map(|line| {
            let line_as_numbers = line
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut combination: usize = 0;
            let mut i = 0;
            for digit_idx in 0..num_digits {
                let largest_number_index = (i..(line_as_numbers.len()
                    - (num_digits - digit_idx - 1)))
                    .reduce(|highest_idx, j| {
                        if line_as_numbers[j] > line_as_numbers[highest_idx] {
                            j
                        } else {
                            highest_idx
                        }
                    })
                    .unwrap();
                i = largest_number_index + 1;
                combination = (combination * 10) + line_as_numbers[largest_number_index];
            }
            combination
        })
        .sum::<usize>();
    dbg!(output);
}
