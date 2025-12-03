pub fn part1() {
    let num_digits: usize = 2;
    let input_str = include_str!("./real_input.txt");
    let output = input_str
        .lines()
        .map(|line| {
            let line_as_numbers = line
                .as_bytes()
                .iter()
                .map(|&b| (b - b'0') as usize)
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

pub fn part2() {
    let num_digits: usize = 12;
    let input_str = include_str!("./real_input.txt");
    let output = input_str
        .lines()
        .map(|line| {
            let line_as_numbers = line
                .as_bytes()
                .iter()
                .map(|&b| (b - b'0') as usize)
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
