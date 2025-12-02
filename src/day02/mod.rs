pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let sum = input_str
        .split(",")
        .map(|s| {
            let (start_s, end_s) = s.split_once('-').unwrap();
            let (start, end) = (
                start_s.trim().parse::<i64>().unwrap(),
                end_s.trim().parse::<i64>().unwrap(),
            );
            let mut sum = 0;
            for i in start..=end {
                let s = &i.to_string();
                if s[0..(s.len() / 2)] == s[(s.len() / 2)..s.len()] {
                    sum += i;
                }
            }
            sum
        })
        .sum::<i64>();
    dbg!(sum);
}

pub fn part2() {}
