use hashbrown::{HashMap, HashSet};

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

    let mut beam_x_positions = HashSet::with_capacity(20);
    beam_x_positions.insert(start_x);

    let mut num_splits = 0;
    for (_, line) in input_iter {
        for (x, char) in line.chars().enumerate() {
            if char == '^' && beam_x_positions.contains(&x) {
                beam_x_positions.remove(&x);
                beam_x_positions.insert(x + 1);
                beam_x_positions.insert(x - 1);
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

    let mut beam_x_positions = HashMap::with_capacity(20);
    beam_x_positions.insert(start_x, 1);

    for (_, line) in input_iter {
        for (x, char) in line.chars().enumerate() {
            if char != '^' {
                continue;
            }
            let num_beams = *beam_x_positions.get(&x).unwrap_or(&0);
            if num_beams < 1 {
                continue;
            }

            beam_x_positions.remove(&x);
            let existing_left = *beam_x_positions.get(&(x - 1)).unwrap_or(&0);
            let existing_rght = *beam_x_positions.get(&(x + 1)).unwrap_or(&0);
            beam_x_positions.insert(x - 1, existing_left + num_beams);
            beam_x_positions.insert(x + 1, existing_rght + num_beams);
        }
    }
    dbg!(&beam_x_positions.values().sum::<usize>());
}
