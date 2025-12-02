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

pub fn part2() {
    let input_str = include_str!("./real_input.txt");
    let sum = input_str
        .split(",")
        .map(|s| {
            let (start_s, end_s) = s.split_once('-').unwrap();
            let (start, end) = (
                start_s.trim().parse::<i64>().unwrap(),
                end_s.trim().parse::<i64>().unwrap(),
            );
            let mut sum: i64 = 0;
            for i in start..=end {
                let s = &i.to_string();
                for pattern_length in 1..(s.len() / 2 + 1) {
                    // Slight speedup from skipping pattern lengths that the string isn't cleanly divisible by
                    if s.len() / pattern_length * pattern_length != s.len() {
                        continue;
                    }
                    // Check if pattern repeats (turns out as_bytes() is way faster than chars().nth() - we don't need to care about unicode)
                    if (1..s.len()).all(|c| s.as_bytes()[c] == s.as_bytes()[c % pattern_length]) {
                        sum += i;
                        break;
                    }
                }
            }
            sum
        })
        .sum::<i64>();
    dbg!(sum);
}
