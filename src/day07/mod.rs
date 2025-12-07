pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let mut input_iter = input_str.lines().enumerate();
    let start_x = input_iter
        .next()
        .unwrap()
        .1
        .chars()
        .enumerate()
        .find(|(_, char)| *char == 'S')
        .unwrap()
        .0;

    let line_length = input_str.lines().next().unwrap().len();
    // index is position, value is number at that position
    let mut beam_x_positions = vec![false; line_length];
    beam_x_positions[start_x] = true;

    let mut num_splits = 0;
    for (_, line) in input_iter {
        for (x, char) in line.chars().enumerate() {
            if char == '^' && beam_x_positions[x] {
                beam_x_positions[x] = false;
                beam_x_positions[x + 1] = true;
                beam_x_positions[x - 1] = true;
                num_splits += 1;
            }
        }
    }
    dbg!(num_splits);
}

pub fn part2() {
    let input_str = include_str!("./real_input.txt");
    let mut input_iter = input_str.lines().enumerate();
    let start_x = input_iter
        .next()
        .unwrap()
        .1
        .chars()
        .enumerate()
        .find(|(_, char)| *char == 'S')
        .unwrap()
        .0;

    let line_length = input_str.lines().next().unwrap().len();
    // index is position, value is number at that position
    let mut beam_x_positions = vec![0; line_length];
    beam_x_positions[start_x] = 1;

    for (_, line) in input_iter {
        for (x, char) in line.chars().enumerate() {
            if char != '^' {
                continue;
            }
            let num_beams = beam_x_positions[x];
            if num_beams < 1 {
                continue;
            }

            beam_x_positions[x] = 0;
            beam_x_positions[x - 1] += num_beams;
            beam_x_positions[x + 1] += num_beams;
        }
    }
    dbg!(&beam_x_positions.iter().sum::<usize>());
}
