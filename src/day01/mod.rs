pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let mut num_zeroes = 0;
    let mut rotation = 50;
    for l in input_str.lines() {
        let (letter, n_str) = l.trim().split_at(1);
        let d_rot = if letter == "L" { -1 } else { 1 } * n_str.parse::<i32>().unwrap();
        rotation = (rotation + d_rot) % 100;
        if rotation == 0 {
            num_zeroes += 1;
        }
    }
    dbg!(num_zeroes);
}

pub fn part2() {
    let input_str = include_str!("./real_input.txt");
    let mut num_zeroes = 0;
    let mut rotation = 50;
    for l in input_str.lines() {
        let (letter, n_str) = l.trim().split_at(1);
        let sign = if letter == "L" { -1 } else { 1 };
        let d_rot_abs = n_str.parse::<i32>().unwrap();
        for _ in 0..d_rot_abs {
            rotation = (rotation + sign) % 100;
            if rotation == 0 {
                num_zeroes += 1;
            }
        }
    }
    dbg!(num_zeroes);
}
