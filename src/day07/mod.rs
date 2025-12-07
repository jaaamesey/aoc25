use hashbrown::HashSet;

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
    for (_, line_original) in input_iter {
        // TODO: Avoid this
        let line = line_original.chars().collect::<Vec<_>>();
        for (x, char) in line.iter().enumerate() {
            if *char == '^' && beam_x_positions.contains(&x) {
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
    let input_str = include_str!("./test_input.txt");
}
